use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

declare_id!("JAVuBXeBZqXNtS73azhBDAoYaaAFfo4gWXoZe2e7Jf8H");

//Initialize vesting contract
// give employer ability to add employer
// allow employer to claim vested tokens



#[program]
pub mod vesting {
    use super::*;

    pub fn create_vesting_account(ctx: Context<CreateVestingAccount>, company_name: String) -> Result<()> {

        *ctx.accounts.vesting_account = VestingAccount {
            owner: ctx.accounts.signer.key(),
            mint: ctx.accounts.mint.key(),
            treasury_token_amount: ctx.accounts.treasury_token_amount.key(),
            company_name: company_name,
            treasury_bump: ctx.bumps.treasury_token_amount,
            bump: ctx.bumps.vesting_account,
        };

        Ok(())
    }

    pub fn create_employee_account(
        ctx: Context<CreateEmployeeAccount>, 
         start_time: i64,
         end_time: i64,
         cliff_time: i64,
         total_amount: u64,
    ) -> Result<()> {

        *ctx.accounts.employee_account = EmployeeAccount {
            beneficiary: ctx.accounts.beneficiary.key(),
            start_time,
            end_time,
            cliff_time,          //time after which employee can start withdrawin
            vesting_amount: ctx.accounts.vesting_account.key(),
            total_amount,
            total_withdrawn: 0,
             bump: ctx.bumps.employee_account,

        };

        Ok(())
    }

}


#[derive(Accounts)]
pub struct CreateEmployeeAccount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub beneficiary: SystemAccount<'info>,

    #[account(
        has_one = owner,
    )]
    pub vesting_account: Account<'info, VestingAccount>,
    #[account(
        init,
        space = 8 + EmployeeAccount::INIT_SPACE,
        payer = owner,
        seeds = [b"employee_vesting", beneficiary.key().as_ref(),
                 vesting_account.key().as_ref()],
        bump,
    )]

    pub employee_account: Account<'info, EmployeeAccount>,

    pub system_program: Program<'info, System>,



}



#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct CreateVestingAccount<'info>  {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        space = 8 + VestingAccount::INIT_SPACE,
        payer = signer,
        seeds = [company_name.as_ref()],
        bump,
    
    )]

    pub vesting_account: Account<'info, VestingAccount>,

    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = signer,
        seeds = [b"vesting_treasury".as_ref(), company_name.as_bytes()],
        bump,
        token::mint = mint,
        token::authority = treasury_token_amount,
    )]

    pub treasury_token_amount: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}


#[account]
#[derive(InitSpace)]
pub struct EmployeeAccount {
    pub beneficiary: Pubkey,
    pub start_time: i64,
    pub end_time: i64,
    pub cliff_time: i64,
    pub vesting_amount: Pubkey,
    pub total_amount: u64,
    pub total_withdrawn: u64,
    pub bump: u8,

}

#[account]
#[derive(InitSpace)]
pub struct VestingAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub treasury_token_amount: Pubkey,
    #[max_len(50)]
    pub company_name: String,
    pub treasury_bump: u8,
    pub bump: u8,
}

