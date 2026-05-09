pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("HGT4DoDJ1crx3HM28t1CJBT2KAAzK44y3esrNWCtP1JE");

#[program]
pub mod soltip {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handle_initialize(ctx)
    }

    pub fn register_creator(ctx: Context<RegisterCreator>, handle: String) -> Result<()> {
        register_creator::handle_register_creator(ctx, handle)
    }
}
