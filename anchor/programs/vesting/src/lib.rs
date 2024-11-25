#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_2022::spl_token_2022::extension::memo_transfer::instruction, token_interface::{Mint, TokenAccount, TokenInterface}};

declare_id!("AsjZ3kWAUSQRNt2pZVeJkywhZ6gpLpHZmJjduPmKZDZZ");

#[program]
pub mod vesting {
    use super::*;

    pub fn create_vesting_account(ctx: Context<CreateVestingAccount>, company_name: String) -> Result<()> {

      *ctx.accounts.vesting_account = VestingAccount {
        owner: ctx.accounts.signer.key(),
        mint: ctx.accounts.mint.key(),
        cold_token_account: ctx.accounts.cold_token_account.key(),
        company_name,
        cold_bump: ctx.bumps.cold_token_account,
        bump: ctx.bumps.vesting_account
      };

      Ok(())
    }

    pub fn create_employee_account(ctx: Context<CreateEmployeeAccount>, start_time: i64, end_time: i64, total_amount: u64, cliff_time: i64) -> Result<()> {
      *ctx.accounts.employee_account = EmployeeAccount {
        beneficiary: ctx.accounts.beneficiary.key(),
        start_time,
        end_time,
        total_amount,
        total_withdrawn: 0,
        cliff_time,
        vesting_account: ctx.accounts.vesting_account.key(),
        bump: ctx.bumps.employee_account
      };

      Ok(())
    }

    pub fn claim_tokens(ctx: Context<ClaimTokens>,company_name: String) {

    }

}




#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct CreateVestingAccount<'info> {
  #[account(mut)]
  pub signer: Signer<'info>,

  #[account(
    init,
    space = 8 + VestingAccount::INIT_SPACE,
    payer = signer,
    seeds = [company_name.as_ref()],
    bump
  )]

  pub vesting_account: Account<'info, VestingAccount>,

  pub mint: InterfaceAccount<'info, Mint>,

  #[account(
    init,
    token::mint = mint,
    token::authority = cold_token_account,
    payer = signer,
    seeds = [b"vesting_cold", company_name.as_bytes()],
    bump
  )]

  pub cold_token_account: InterfaceAccount<'info, TokenAccount>,

  pub system_program: Program<'info, System>,
  pub token_program: Interface<'info, TokenInterface>

}
#[derive(Accounts)]
pub struct CreateEmployeeAccount<'info> {
  #[account(mut)]
  pub owner: Signer<'info>,
  pub beneficiary: SystemAccount<'info>,

  #[account(
    has_one = owner
  )]
  pub vesting_account: Account<'info, VestingAccount>,

  #[account(
    init,
    space = 8 + EmployeeAccount::INIT_SPACE,
    payer = owner,
    seeds = [b"employee_vesting", beneficiary.key().as_ref(), vesting_account.key().as_ref()],
    bump
  )]

  pub employee_account: Account<'info, EmployeeAccount>,

  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct ClaimTokens<'info> {
  #[account(mut)]
  pub beneficiary: Signer<'info>,

  #[account(
    mut,
    seeds = [b"employee_vesting", beneficiary.key().as_ref(), vesting_account.key().as_ref()],
    bump = employee_account.bump,
    has_one = beneficiary,
    has_one = vesting_account
  )]
  pub employee_account: Account<'info, EmployeeAccount>,

  #[account(
    mut,
    seeds = [company_name.as_ref()],
    bump = vesting_account.bump,
    has_one = cold_token_account,
    has_one = mint
  )]
  pub vesting_account: Account<'info, VestingAccount>,
  pub mint: InterfaceAccount<'info, Mint>,

  #[account(mut)]
  pub cold_token_account: InterfaceAccount<'info, TokenAccount>,

  #[account(
    init_if_needed,
    payer = beneficiary,
    associated_token::mint = mint,
    associated_token::authority = beneficiary,
    associated_token::token_program = token_program
  )]
  pub employee_token_account: InterfaceAccount<'info, TokenAccount>,
  pub token_program: Interface<'info, TokenInterface>,
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>

}



#[account]
#[derive(InitSpace)]
pub struct VestingAccount {
  pub owner: Pubkey,
  pub mint: Pubkey,
  pub cold_token_account: Pubkey,
  #[max_len(50)]
  pub company_name: String,
  pub cold_bump: u8,
  pub bump: u8
}

#[account]
#[derive(InitSpace)]
pub struct EmployeeAccount {
  pub beneficiary: Pubkey,
  pub start_time: i64,
  pub end_time: i64,
  pub cliff_time: i64,
  pub vesting_account: Pubkey,
  pub total_amount: u64,
  pub total_withdrawn: u64,
  pub bump: u8
}