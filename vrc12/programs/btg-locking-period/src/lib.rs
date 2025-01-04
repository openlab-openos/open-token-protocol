use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token_interface::Mint;

declare_id!("99mpy2LmaD747bwWSLs5sNHPuRRPMdd1aR22HeAceqUh");

#[program]
pub mod btg_locking_period {
    use super::*;
    pub fn lock(ctx: Context<Lock>, amount: u64, end_time: i64) -> Result<()> {
        let clock = Clock::get()?;
        require!(
            end_time > clock.unix_timestamp + 86400,
            ErrorCode::UnlockTimeTooEarly
        );
        require!(
            end_time < clock.unix_timestamp + 10 * 365 * 86400,
            ErrorCode::UnlockTimeTooLate
        );

        //check mint account
        require!(
            ctx.accounts.mint.is_initialized == true,
            ErrorCode::MintNotInitialized
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

        require!(
            ctx.accounts.lock_account.is_unlocked == false,
            ErrorCode::AccountAlreadyUnlocked,
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
    pub mint: InterfaceAccount<'info, Mint>,
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
    #[msg("Unlock time is smaller than 24 hours")]
    UnlockTimeTooEarly,
    #[msg("Unlock time has not been reached")]
    UnlockTimeNotReached,
    #[msg("Unauthorized access.")]
    Unauthorized,
    #[msg("Invalid mint account")]
    InvalidMintAccount,
    #[msg("Mint account is not initialized")]
    MintNotInitialized,
    #[msg("Account is already unlocked")]
    AccountAlreadyUnlocked,
    #[msg("Unlock time is larger than 10 years")]
    UnlockTimeTooLate,
    
}
