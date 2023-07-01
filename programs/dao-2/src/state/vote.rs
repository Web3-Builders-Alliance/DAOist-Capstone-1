use crate::{constants::*, errors::DaoError, accounts::Vote};
use anchor_lang::prelude::*;

#[account]
pub struct VoteState {
    pub owner: Pubkey,
    pub amount: u64,
    pub choice: VoteChoice,
    //pub nft: vec<Pubkey>
    pub bump: u8
}

impl VoteState {
    pub const LEN: usize = 8 + 1 + PUBKEY_L + U64_L + U8_L;

    pub fn init(
        &mut self,
        owner: Pubkey,
        amount: u64,
        choice: VoteChoice,
        bump: u8,
    ) -> Result<()> {
        self.owner = owner;
        self.amount = amount;
        self.choice = choice;
        self.bump = bump;
        Ok(())
    }
}


#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum VoteChoice {
    For,
    Against,
    Abstain,
}

