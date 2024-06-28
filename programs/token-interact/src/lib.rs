use anchor_lang::prelude::*;

declare_id!("EZi3sKTYVbuCAWDq81fFC1pvTTSGy8SUbMVuQn9DGiYF");

#[program]
pub mod token_interact {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
