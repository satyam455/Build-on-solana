use anchor_lang::prelude::*;

declare_id!("JAVuBXeBZqXNtS73azhBDAoYaaAFfo4gWXoZe2e7Jf8H");

//Initialize vesting contract
// give employer ability to add employer
// allow employer to claim vested tokens

#[program]
pub mod vesting {
    use super::*;
}

#[derive(Accounts)]
pub struct Initialize {}
