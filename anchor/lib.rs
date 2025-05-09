use anchor_lang::prelude::*;
use solana_program::keccak::hashv;

declare_id!("YourProgramIDGoesHere"); // Replace with your Solana program ID later

#[program]
pub mod killo_energy {
    use super::*;

    pub fn submit_energy(ctx: Context<SubmitEnergy>, energy: u64, nonce: u64) -> Result<()> {
        let energy_account = &mut ctx.accounts.energy_account;

        // Compute hash for tamper-proof verification
        let energy_data = energy.to_le_bytes();
        let nonce_data = nonce.to_le_bytes();
        let energy_hash = hashv(&[&energy_data, &nonce_data]).0;

        // Store energy value and hash
        energy_account.energy = energy;
        energy_account.energy_hash = energy_hash;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SubmitEnergy<'info> {
    #[account(init_if_needed, payer = user, space = 8 + 8 + 32)]
    pub energy_account: Account<'info, EnergyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct EnergyAccount {
    pub energy: u64,
    pub energy_hash: [u8; 32],
}
