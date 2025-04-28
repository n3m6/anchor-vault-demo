use anchor_lang::prelude::*;
use crate::VaultState;
use crate::DISCRIMINATOR;

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