use crate::{constants::*, errors::DaoError, accounts::Vote};
use anchor_lang::prelude::*;

#[account]
pub struct VoteState {
    pub owner: Pubkey,
    /* pub vote : VoteType, */
    pub amount: u64,
    //pub nft: vec<Pubkey>
    pub bump: u8
}

impl VoteState {
    pub const LEN: usize = 8 + PUBKEY_L + U64_L + U8_L;

    pub fn init(
        &mut self,
        owner: Pubkey,
        /* vote: VoteType, */
        amount: u64,
        bump: u8,
    ) -> Result<()> {
        self.owner = owner;
        /* self.vote = vote; */
        self.amount = amount;
        self.bump = bump;
        Ok(())
    }
}



