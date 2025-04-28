#![allow(unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::system_program::{transfer, Transfer};
use anchor_lang::{prelude::*, program};

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2cXAisoiUFiQFmY1MMf4CGgTqXRU3sNJmzLsV9vX7mpB");

#[program]
pub mod anchor_vault {
    use super::*;
    // Initialize and store the bumps in vault_state
    pub fn init(ctx: Context<Init>) -> Result<()> {
        ctx.accounts.vault_state.set_inner(VaultState {
            vault_bump: ctx.bumps.vault,
            state_bump: ctx.bumps.vault_state,
        });

        Ok(())
    }

    pub fn deposit(ctx: Context<VaultTransfer>, amount: u64) -> Result<()> {
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: ctx.accounts.user.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, amount)
    }

    pub fn withdraw(ctx: Context<VaultTransfer>, amount: u64) -> Result<()> {
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.user.to_account_info(),
        };

        let binding = ctx.accounts.user.key();
        let seeds = &[
            b"vault",
            binding.as_ref(),
            &[ctx.accounts.vault_state.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount)
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        let amount = ctx.accounts.vault.to_account_info().lamports();

        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.user.to_account_info(),
        };

        let seeds = &[
            b"vault",
            ctx.accounts.vault_state.to_account_info().key.as_ref(),
            &[ctx.accounts.vault_state.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount)
    }
}
