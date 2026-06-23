use anchor_lang::prelude::*;

use crate::NewAccount;

#[derive(Accounts)]
pub struct Double<'info> {
    #[account(mut)]
    pub account: Account<'info, NewAccount>,
    pub signer: Signer<'info>,
}

pub(crate) fn handler(ctx: Context<Double>) -> Result<()> {
    ctx.accounts.account.data = ctx.accounts.account.data * 2;
    Ok(())
}
