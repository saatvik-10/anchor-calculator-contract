use anchor_lang::prelude::*;

#[account]
pub struct NewAccount {
    pub data: u32,
}
