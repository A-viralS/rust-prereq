use anchor_lang::prelude::*;

declare_id!("7fZ9sAtsbD7NGn75VDqJCCNcFuvvMxtXt3LxXxF1RQS7");

#[program]
mod enroll {
    use super::*;

    pub fn enroll(ctx: Context<Enroll>, data: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Enroll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
