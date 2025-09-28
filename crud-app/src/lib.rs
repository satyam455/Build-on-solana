#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("Count3AcZucFDPSFBAeHkQ6AvttieKUkyJ8HiQGhQwe");

#[program]
pub mod crud {
    use super::*;

    pub fn create_journal(ctx: Context<CreateEntry>, title: String, message: String) -> Result<()> {
        let journal_entry = &mut ctx.accounts.journal_entry;

        journal_entry.owner = ctx.accounts.owner.key();
        journal_entry.title = title;
        journal_entry.message = message;

        Ok(())
    }

    pub fn update_journal(ctx: Context<UpdateEntry>, title: String, message: String) -> Result<()> {
        let journal_update = &mut ctx.accounts.journal_entry;

        journal_update.title = title;
        journal_update.message = message;

        Ok(())
    }

    pub fn delete_journal(ctx: Context<DeleteEntry>, title: String, message: String) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateEntry<'info> {
    #[account(
        init,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        space = 8 + JournalEntry::INIT_SPACE,
        payer = owner,
    )]
    pub journal_entry: Account<'info, JournalEntry>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct UpdateEntry<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        realloc = 8 + JournalEntry::INIT_SPACE, // reallocates the space in the account that decreases and increases the rent accordingly
        realloc::payer = owner,   // it reallocates the payer of account accoording to the space the account has token and pay rent for that space
        realloc::zero = true,   // it zeros the space and reallocate the space according to the update made.
    )]
    pub journal_entry: Account<'info, JournalEntry>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteEntry<'info> {
    #[account(
        mut,
         seeds = [title.as_bytes(), owner.key().as_ref()],
          bump,
          close = owner, // it closes the account 
        )]
    pub journal_entry: Account<'info, JournalEntry>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct JournalEntry {
    pub owner: Pubkey,
    #[max_len(50)]
    pub title: String,
    #[max_len(280)]
    pub message: String,
}
