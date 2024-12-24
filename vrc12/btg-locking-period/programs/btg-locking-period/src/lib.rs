use anchor_lang::prelude::*;

declare_id!("9aBFyjj5mw9RVQZc8AVoENMzMiYLoY4hdZ8PvtDdqoNM");

#[program]
pub mod btg_locking_period {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
