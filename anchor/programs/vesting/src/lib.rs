#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("AsjZ3kWAUSQRNt2pZVeJkywhZ6gpLpHZmJjduPmKZDZZ");

#[program]
pub mod vesting {
    use super::*;

    pub fn createVestingAccount(ctx: Context<CreateVestingAccount>) -> Result<()> {

      Ok(())
    }

}


#[derive(Accounts)]
pub struct CreateVestingAccount<'info> {
  #[account(mut)]
  pub signer: Signer<'info>

  #[account(
    init,
    space = 8 + 
  )]
}

#[account]
pub struct VestingAccount {
  pub owner: Pubkey,
  pub mint: Pubkey,
  pub cold_token_account: Pubkey,
  pub company_name: String,
  pub treasury_bump: u8,
  pub bump: u8
}