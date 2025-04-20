#![allow(unexpected_cfgs)]
use anchor_lang::system_program::{transfer, Transfer};
use anchor_lang::{prelude::*, program};

declare_id!("GbxPsihGFBGrtw7qocpjbNbJDEBMVRQLTUaxoH4D7RnP");

#[program]
pub mod anchor_vault_demo {
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
}

const DISCRIMINATOR: usize = 8;

#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        seeds = [b"state", user.key().as_ref()],
        bump,
        space = DISCRIMINATOR + VaultState::INIT_SPACE,
    )]
    pub vault_state: Account<'info, VaultState>, // storing info about the vault here

    #[account(
        seeds = [b"vault", vault_state.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>, // actual vault

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VaultTransfer<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
        close = user,
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}
