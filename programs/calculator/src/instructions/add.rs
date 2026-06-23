use anchor_lang::prelude::*;

use crate::NewAccount;

#[derive(Accounts)]
pub struct Add<'info> {
    #[account(mut)]
    pub account: Account<'info, NewAccount>,
    pub signer: Signer<'info>,
}

pub(crate) fn handler(ctx: Context<Add>, value: u32) -> Result<()> {
    ctx.accounts.account.data = ctx.accounts.account.data + value;
    Ok(())
}
