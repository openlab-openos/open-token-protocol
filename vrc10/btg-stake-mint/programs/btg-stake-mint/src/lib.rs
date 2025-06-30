use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::mint_to;
use anchor_spl::token_interface::Mint;
use anchor_spl::token_interface::MintTo;
use anchor_spl::token_interface::TokenAccount;
use anchor_spl::token_interface::TokenInterface;

declare_id!("13LuL7scpzXTa5hCgs7LYpeTfpcdNcU9DkuW1rMqHYQU");

#[program]

pub mod btg_stake_mint {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.owner = *ctx.accounts.authority.key;
        config.tokens = Vec::new();
        Ok(())
    }

    pub fn add_to_whitelist(
        ctx: Context<Initialize>,
        symbol: String,
        mint: Pubkey,
        output_rate: u64,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        require!(config.owner == ctx.accounts.authority.key(), ErrorCode::Unauthorized);

        // 检查是否已存在
        if config.tokens.iter().any(|token| token.mint == mint) {
            return Err(ErrorCode::TokenAlreadyExists.into());
        }
        config.tokens.push(TokenInfo {
            symbol: symbol,
            mint: mint,
            output_rate: output_rate,
        });
        Ok(())
    }

    pub fn remove_from_whitelist(ctx: Context<Initialize>, mint: Pubkey) -> Result<()> {
        let config = &mut ctx.accounts.config;
        require!(config.owner == ctx.accounts.authority.key(), ErrorCode::Unauthorized);

        if let Some(index) = config.tokens.iter().position(|t| t.mint == mint) {
            config.tokens.remove(index);
        }
        Ok(())
    }

    pub fn stake_btg(ctx: Context<StakeBtg>, amount: u64) -> Result<()> {
        if amount < 1_000_000 {
            return Err(ErrorCode::AmountTooSmall.into());
        }
        if ctx.accounts.user.lamports() < amount {
            return Err(ErrorCode::InsufficientFunds.into());
        }
        let config = &ctx.accounts.config;
        require!(
            config
                .tokens
                .iter()
                .any(|token| token.mint == ctx.accounts.token_mint.key()),
            ErrorCode::InvalidToken
        );

        let token_info = config
            .tokens
            .iter()
            .find(|t| t.mint == ctx.accounts.token_mint.key())
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
        let tokens_to_mint = amount
            .checked_mul(token_info.output_rate).expect("Math error")
            .checked_div(1_000_000_000).expect("Math error"); // 假设输出为十亿分之一单位

        let signer_seeds: &[&[&[u8]]] = &[&[b"config"]];
        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.token_mint.to_account_info(),
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
        staking_vault.mint = ctx.accounts.token_mint.key();
        staking_vault.btg_amount = amount;
        staking_vault.price = 1;
        staking_vault.output_rate = token_info.output_rate;
        staking_vault.output_token_amount = tokens_to_mint;
        staking_vault.time = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn redeem_token(ctx: Context<RedeemToken>) -> Result<()> {
        let staking_vault = &ctx.accounts.staking_vault;

        // Check that the token is on the whitelist
        let config = &ctx.accounts.config;

        // 1. 检查 token_mint 是否在白名单中
        require!(
            config
                .tokens
                .iter()
                .any(|token| token.mint == staking_vault.mint),
            ErrorCode::InvalidToken
        );

        // 2. 验证 staking_vault 中的 btg_amount 不为零
        require!(staking_vault.btg_amount > 0, ErrorCode::InsufficientFunds);

        // 3. 验证 stake_record 的用户是否与调用者匹配（可选）
        require!(staking_vault.user == ctx.accounts.user.key(), ErrorCode::Unauthorized);

        // 5. 验证 vault 中的 lamports 是否足够支付赎回的 BTG
        let vault_lamports = ctx.accounts.staking_vault.get_lamports();
        require!(
            vault_lamports >= staking_vault.btg_amount,
            ErrorCode::InsufficientFunds
        );
        let user_key = ctx.accounts.user.key(); // 获取 Pubkey 值
        let tokne_mint = ctx.accounts.token_mint.key();

        let signer_seeds: &[&[&[u8]]] = &[&[user_key.as_ref(),tokne_mint.as_ref(),&[ctx.bumps.staking_vault]]];
        system_program::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.staking_vault.to_account_info(),
                    to: ctx.accounts.user.to_account_info(),
                },
                signer_seeds, // 使用 seeds 让 PDA 签名
            ),
            vault_lamports,
        )?;


    //     let signer_seeds: &[&[&[u8]]] = &[&[b"token", &[ctx.bumps.sender_token_account]]];
 
    // let amount = ctx.accounts.sender_token_account.amount;
    // let decimals = ctx.accounts.mint.decimals;
 
    // let cpi_accounts = TransferChecked {
    //     mint: ctx.accounts.mint.to_account_info(),
    //     from: ctx.accounts.sender_token_account.to_account_info(),
    //     to: ctx.accounts.recipient_token_account.to_account_info(),
    //     authority: ctx.accounts.sender_token_account.to_account_info(),
    // };
    // let cpi_program = ctx.accounts.token_program.to_account_info();
    // let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(signer_seeds);
    // token_interface::transfer_checked(cpi_context, amount, decimals)?;
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
pub struct StakeBtg<'info> {
    #[account(
        init,
        seeds = [user.key().as_ref(), token_mint.key().as_ref()],
        bump,
        payer = user,
        space = 8 + StakingVault::INIT_SPACE
    )]
    pub staking_vault: Account<'info, StakingVault>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub token_mint: InterfaceAccount<'info, Mint>,
    #[account(seeds = [b"config"],bump)]
    pub config: Account<'info, StakeConfig>,

    #[account(
        init,
        payer = user,
        associated_token::mint = token_mint,
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
    #[account(
        mut,
        seeds = [user.key().as_ref(), token_mint.key().as_ref()],
        bump,
        close = user
    )]
    pub staking_vault: Account<'info, StakingVault>,
    #[account(seeds = [b"config"],bump)]
    pub config: Account<'info, StakeConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account()]
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct StakeConfig {
    pub owner: Pubkey,
    #[max_len(100)]
    pub tokens: Vec<TokenInfo>,
}

#[derive(InitSpace,Clone, Debug,AnchorDeserialize,AnchorSerialize)]
pub struct TokenInfo {
    #[max_len(10)] // 添加max_len属性以满足程序宏的要求
    pub symbol: String,
    pub mint: Pubkey,
    pub output_rate: u64,
}
#[account]
#[derive(InitSpace)]
pub struct StakingVault {
    pub user: Pubkey,
    pub btg_amount: u64,
    pub mint: Pubkey,
    pub price: u64,
    pub output_rate: u64,
    pub output_token_amount: u64,
    pub time: i64,
}

// Custom errors
#[error_code]
pub enum ErrorCode {
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
}
