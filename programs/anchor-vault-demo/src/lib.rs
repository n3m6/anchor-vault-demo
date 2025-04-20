use anchor_lang::prelude::*;

declare_id!("GbxPsihGFBGrtw7qocpjbNbJDEBMVRQLTUaxoH4D7RnP");

#[program]
pub mod anchor_vault_demo {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
