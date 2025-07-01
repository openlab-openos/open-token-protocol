use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_option::COption;
use anchor_lang::system_program;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::mint_to;
use anchor_spl::token_interface::Mint;
use anchor_spl::token_interface::MintTo;
use anchor_spl::token_interface::TokenAccount;
use anchor_spl::token_interface::TokenInterface;
use anchor_spl::token_interface::{burn, Burn};

declare_id!("CmW1X4qWBKTTYwCPPZymzCkcgLJtS5miKyhVgAxWVmKD");

#[program]
pub mod btg_stake_mint {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.authority = *ctx.accounts.authority.key;
        config.tokens = Vec::new();
        Ok(())
    }

    pub fn add_to_whitelist(
        ctx: Context<WhiteList>,
        symbol: String,
        output_rate: f64,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        require!(
            config.authority == ctx.accounts.authority.key(),
            MyErrorCode::Unauthorized
        );

        let mint = ctx.accounts.mint.key();

        require!(
            ctx.accounts.mint.mint_authority == COption::Some(config.key()),
            MyErrorCode::InvalidMintAuthority
        );
        // 检查是否已存在
        if config.tokens.iter().any(|token| token.mint == mint) {
            return Err(MyErrorCode::TokenAlreadyExists.into());
        }

        config.tokens.push(TokenInfo {
            symbol,
            mint,
            output_rate,
        });
        Ok(())
    }

    pub fn remove_from_whitelist(ctx: Context<WhiteList>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        let mint = ctx.accounts.mint.key();
        require!(
            config.authority == ctx.accounts.authority.key(),
            MyErrorCode::Unauthorized
        );

        if let Some(index) = config.tokens.iter().position(|t| t.mint == mint) {
            config.tokens.remove(index);
        }
        Ok(())
    }

    pub fn stake_btg(ctx: Context<StakeBtg>, amount: u64) -> Result<()> {
        if amount < 1_000_000 {
            return Err(MyErrorCode::AmountTooSmall.into());
        }
        if ctx.accounts.user.lamports() < amount {
            return Err(MyErrorCode::InsufficientFunds.into());
        }
        let config = &ctx.accounts.config;
        require!(
            config
                .tokens
                .iter()
                .any(|token| token.mint == ctx.accounts.mint.key()),
            MyErrorCode::InvalidToken
        );

        let token_info = config
            .tokens
            .iter()
            .find(|t| t.mint == ctx.accounts.mint.key())
            .unwrap();

        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.user.to_account_info(),
                    to: ctx.accounts.staking_vault.to_account_info(),
                },
            ),
            amount,
        )?;

        // 4. 计算铸造数量：amount * output_rate / 1e9 （根据你的精度调整）
        let price = (1.0 * 1e9) as u64;
        let output_rate_fixed = (token_info.output_rate * 1e9) as u64; // 放大 1e9 倍
        let tokens_to_mint = amount
            .checked_mul(price)
            .expect("Math error")
            .checked_div(1_000_000_000) // 缩小回整数单位
            .expect("Math error")
            .checked_mul(output_rate_fixed)
            .expect("Math error")
            .checked_div(1_000_000_000) // 缩小回整数单位
            .expect("Math error");

        let signer_seeds: &[&[&[u8]]] = &[&[b"config", &[ctx.bumps.config]]];
        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.config.to_account_info(),
                },
                signer_seeds,
            ),
            tokens_to_mint,
        )?;

        // Create stake record
        let staking_vault = &mut ctx.accounts.staking_vault;
        staking_vault.user = *ctx.accounts.user.key;
        staking_vault.mint = ctx.accounts.mint.key();
        staking_vault.btg_amount = amount;
        staking_vault.price = 1.0;
        staking_vault.output_rate = token_info.output_rate;
        staking_vault.output_token_amount = tokens_to_mint;
        staking_vault.time = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn redeem_token(ctx: Context<RedeemToken>) -> Result<()> {
        let staking_vault = &ctx.accounts.staking_vault;

        // 2. 验证 staking_vault 中的 btg_amount 不为零
        require!(staking_vault.btg_amount > 0, MyErrorCode::InsufficientFunds);

        // 3. 验证 stake_record 的用户是否与调用者匹配（可选）
        require!(
            staking_vault.user == ctx.accounts.user.key(),
            MyErrorCode::Unauthorized
        );

        // 5. 验证 vault 中的 lamports 是否足够支付赎回的 BTG
        let vault_lamports = ctx.accounts.staking_vault.get_lamports();
        require!(
            vault_lamports >= staking_vault.btg_amount,
            MyErrorCode::InsufficientFunds
        );

        let token_mint = ctx.accounts.mint.key();
        require!(
            token_mint == ctx.accounts.staking_vault.mint,
            MyErrorCode::InvalidToken
        );

        let user_token_account = &ctx.accounts.user_token_account;
        let tokens_to_burn = staking_vault.output_token_amount;
        require!(
            user_token_account.amount >= tokens_to_burn,
            MyErrorCode::InsufficientFunds
        );

        // 销毁用户的代币
        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        burn(cpi_context, tokens_to_burn)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [b"config"],
        bump,
        payer = authority,
        space = 8 + StakeConfig::INIT_SPACE
    )]
    pub config: Account<'info, StakeConfig>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct WhiteList<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump,
    )]
    pub config: Account<'info, StakeConfig>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account()]
    pub mint: InterfaceAccount<'info, Mint>,
}
#[derive(Accounts)]
pub struct StakeBtg<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + StakingVault::INIT_SPACE
    )]
    pub staking_vault: Account<'info, StakingVault>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(seeds = [b"config"],bump)]
    pub config: Account<'info, StakeConfig>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct RedeemToken<'info> {
    #[account(mut , close = user)]
    pub staking_vault: Account<'info, StakingVault>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[account]
#[derive(InitSpace)]
pub struct StakeConfig {
    pub authority: Pubkey,
    #[max_len(100)]
    pub tokens: Vec<TokenInfo>,
}

#[derive(InitSpace, Clone, AnchorDeserialize, AnchorSerialize)]
pub struct TokenInfo {
    #[max_len(10)] // 添加max_len属性以满足程序宏的要求
    pub symbol: String,
    pub mint: Pubkey,
    pub output_rate: f64,
}
#[account]
#[derive(InitSpace)]
pub struct StakingVault {
    pub user: Pubkey,
    pub btg_amount: u64,
    pub mint: Pubkey,
    pub price: f64,
    pub output_rate: f64,
    pub output_token_amount: u64,
    pub time: i64,
}

// Custom errors
#[error_code]
pub enum MyErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Invalid token - not in whitelist")]
    InvalidToken,
    #[msg("The token already exists in the list.")]
    TokenAlreadyExists,
    #[msg("The provided mint address is invalid.")]
    InvalidMintAddress,
    #[msg("The staking amount is too small.")]
    AmountTooSmall,
    #[msg("The user does not have enough funds.")]
    InsufficientFunds,
    #[msg("The mint authority is not controlled by the program.")]
    InvalidMintAuthority,
}
