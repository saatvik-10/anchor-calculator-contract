pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("64mU9SLvqv8texzRsRMKgG13kRXx1QkXPHAJ9jBVhDUL");

#[program]
pub mod calculator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn halve(ctx: Context<Halve>) -> Result<()> {
        halve::handler(ctx)
    }

    pub fn double(ctx: Context<Double>) -> Result<()> {
        double::handler(ctx)
    }

    pub fn add(ctx: Context<Add>, value: u32) -> Result<()> {
        add::handler(ctx, value)
    }

    pub fn subtract(ctx: Context<Subtract>, value: u32) -> Result<()> {
        subtract::handler(ctx, value)
    }
}
