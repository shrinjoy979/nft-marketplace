use anchor_lang::prelude::*;

declare_id!("CPjxsBYJZjVmYXo51tVHKuvZmAjgwS9doRFtB8rRXZ8v");

pub mod instructions;
pub mod states;

pub use instructions::*;

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize_marketplace(
        ctx: Context<InitializeMarketplace>,
        fee_percentage: u8
    ) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
