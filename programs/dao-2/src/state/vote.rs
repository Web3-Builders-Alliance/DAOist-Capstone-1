use crate::{constants::*, errors::DaoError, accounts::Vote};
use anchor_lang::prelude::*;

#[account]
pub struct VoteState {
    pub owner: Pubkey,
    pub amount: u64,
    pub choice: u8,
    pub bump: u8
}

impl VoteState {
    pub const LEN: usize = 8 + 1 + PUBKEY_L + U64_L + 2 * U8_L;

    pub fn init(
        &mut self,
        owner: Pubkey,
        amount: u64,
        choice: u8,
        bump: u8,
    ) -> Result<()> {
        self.owner = owner;
        self.amount = amount;
        self.choice = choice;
        self.bump = bump;
        Ok(())
    }
}

