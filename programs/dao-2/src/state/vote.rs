use crate::{constants::*, errors::DaoError, accounts::Vote};
use anchor_lang::prelude::*;

#[account]
pub struct VoteState {
    pub owner: Pubkey,
    /* pub votetype : VoteType, */
    pub amount: u64,
    //pub nft: vec<Pubkey>
    pub bump: u8
}

impl VoteState {
    pub const LEN: usize = 8 + PUBKEY_L + U64_L + U8_L;

    pub fn init(
        &mut self,
        owner: Pubkey,
        /* votetype: VoteType, */
        amount: u64,
        bump: u8,
    ) -> Result<()> {
        self.owner = owner;
        /* self.votetype = votetype; */
        self.amount = amount;
        self.bump = bump;
        Ok(())
    }
}

/* #[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
pub enum VoteType {
    Single, // Single Vote
    Multiple, // Multiple Vote

} */