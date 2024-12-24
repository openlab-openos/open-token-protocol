use anchor_lang::prelude::*;
use anchor_lang::system_program;
declare_id!("9aBFyjj5mw9RVQZc8AVoENMzMiYLoY4hdZ8PvtDdqoNM");

#[program]
pub mod btg_locking_period {
    use super::*;
    pub fn lock(ctx: Context<Lock>, amount: u64, end_time: i64) -> Result<()> {
        let clock = Clock::get()?;
        require!(
            end_time > clock.unix_timestamp,
            ErrorCode::UnlockTimeTooEarly
        );
        //check mint account 
        let mint_program_id = ctx.accounts.mint.to_account_info().owner.to_string();
        require!(
            mint_program_id == "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
                || mint_program_id == "Token9ADbPtdFC3PjxaohBLGw2pgZwofdcbj6Lyaw6c",
            ErrorCode::InvalidMintAccount
        );

        ctx.accounts.lock_account.amount = amount;
        ctx.accounts.lock_account.start_time = clock.unix_timestamp;
        ctx.accounts.lock_account.end_time = end_time;
        ctx.accounts.lock_account.owner = *ctx.accounts.owner.key;
        ctx.accounts.lock_account.is_unlocked = false;
        ctx.accounts.lock_account.mint = ctx.accounts.mint.key();

        // Transfer BTG to the lock account
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.owner.to_account_info(),
                    to: ctx.accounts.lock_account.to_account_info(),
                },
            ),
            amount,
        )?;

        msg!(
            "lock {} lamports in address {} with unlock time {}",
            amount,
            ctx.accounts.lock_account.key().to_string(),
            end_time
        );
        Ok(())
    }

    pub fn unlock(ctx: Context<Unlock>) -> Result<()> {
        let clock = Clock::get()?;
        //time check
        require!(
            clock.unix_timestamp >= ctx.accounts.lock_account.end_time,
            ErrorCode::UnlockTimeNotReached
        );
        //owner check
        require!(
            ctx.accounts.lock_account.owner == *ctx.accounts.owner.key,
            ErrorCode::Unauthorized
        );
        ctx.accounts.lock_account.is_unlocked = true;

        // Transfer BTG back to the owner
        let amount = ctx.accounts.lock_account.amount;

        **ctx
            .accounts
            .lock_account
            .to_account_info()
            .try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.owner.try_borrow_mut_lamports()? += amount;

        msg!("unlock {} lamports", amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Lock<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 32 + 8 + 8 + 8 + 1)]
    pub lock_account: Account<'info, LockAccount>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Unlock<'info> {
    #[account(mut, has_one = owner , owner = id())]
    pub lock_account: Account<'info, LockAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Mint {
    pub mint_authority: Option<Pubkey>,
    pub supply: u64,
    pub decimals: u8,
    pub is_initialized: bool,
    pub freeze_authority: Option<Pubkey>,
}

#[account]
pub struct LockAccount {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub is_unlocked: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unlock time is too early.")]
    UnlockTimeTooEarly,
    #[msg("Unlock time has not been reached.")]
    UnlockTimeNotReached,
    #[msg("Unauthorized access.")]
    Unauthorized,
    #[msg("Invalid mint account")]
    InvalidMintAccount,
}
