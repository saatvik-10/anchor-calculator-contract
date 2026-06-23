use anchor_lang::prelude::*;

use crate::NewAccount;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 4)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn handler(ctx: Context<Initialize>) -> Result<()> {
    ctx.accounts.new_account.data = 1;
    Ok(())
}
