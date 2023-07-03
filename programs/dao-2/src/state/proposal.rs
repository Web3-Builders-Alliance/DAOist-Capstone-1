use crate::{constants::*, errors::DaoError, accounts::Vote};
use anchor_lang::prelude::*;

use super::{VoteOption, VoteChoice, DaoConfig};

#[account]
pub struct Proposal {
    pub id: u64, // A unique ID. Can be sequential or random.
    pub name: String, // A proposal name
    pub gist: String, // 72 bytes (39 bytes + / + 32 char ID)
    pub proposal: ProposalType,
    pub vote_type: VoteType,
    pub result: ProposalStatus,
    pub quorum: u8,
    pub threshold: u64,
    pub votes: u64,
    pub expiry: u64,
    pub bump: u8,
    pub created_time: i64,
    pub vote_counts: [u64; 3],
}

impl Proposal {
    pub const LEN: usize = 8 + 32 + 72 + ENUM_L * 2 + U8_L * 2 + 3 * U64_L + U8_L;
    pub fn init(
        &mut self,
        id: u64,
        name: String,
        gist: String,
        proposal: ProposalType,
        vote_type: VoteType,
        quorum: u8,
        threshold:u64,
        expiry: u64,
        bump: u8  
    ) -> Result<()> {
        require!(name.len() < 33, DaoError::InvalidName);
        require!(gist.len() < 73, DaoError::InvalidGist);
        self.id = id;
        self.proposal = proposal;
        self.vote_type = VoteType::SingleChoice;
        self.name = name;
        self.gist = gist;
        self.result = ProposalStatus::PreVoting;
        self.quorum = quorum;
        self.threshold = threshold;
        self.votes = 0;
        self.bump = bump;
        self.expiry = Clock::get()?.slot.checked_add(expiry).ok_or(DaoError::Overflow)?;
        self.created_time = Clock::get()?.slot;
        self.vote_counts = [0, 0, 0];
        Ok(())

    }




        pub fn is_single_choice(
        &self
    ) -> Result<()> {
        require!(self.vote_type == VoteType::SingleChoice, DaoError::InvalidVoteType);
        Ok(())
    }

   
    pub fn is_multi_choice(
        &self
    ) -> Result<()> {
        require!(self.vote_type == VoteType::MultipleChoice, DaoError::InvalidVoteType);
        Ok(())
    }

}
    // transition from PreVoting to Open  -- WOULD THIS RE OPEN FAILED OR SUCESSED 
     pub fn try_initialize(
        &mut self,
        config:  &DaoConfig
    ) { 
        self.is_votable()?;
        let mut current_time = Clock::get()?.slot;
        let required_time = self.created_time + config.prevoting_period;

        require!(current_time >= required_time, DaoError::InvalidRequiredTime);
        self.result = ProposalStatus::Open; 
        
    }



     // let quorum:u128 = 100000 as u128 * 45 as u128 / 100 as u128;
    /*    let mut quorum:u128 = 100000 as u128 * 45 as u128 / 100 as u128; 
    quorum = quorum as u64; */

    pub fn try_finalize(
        &mut self
    ) {

        //Calculate nr of votes to achieve quorum
        let quorum:u128 = (self.votes - self.vote_counts[2]) * ( self.quorum / 100 );   
        if self.votes >= self.threshold && self.vote_counts[0] >= quorum && self.check_expiry().is_ok() {
            self.result = ProposalStatus::Succeeded
        } else if self.votes < self.threshold && self.check_expiry().is_err() || self.vote_counts[1] >= quorum {
            self.result = ProposalStatus::Failed
        }
    }

    pub fn check_expiry(
        &mut self
    ) -> Result<()> {
        require!(Clock::get()?.slot < self.expiry, DaoError::Expired);
        Ok(())
    }

    pub fn is_votable(
        &mut self
    ) -> Result<()> {
        require!(self.result == ProposalStatus::PreVoting, DaoError::InvalidProposalStatus);
        Ok(())
    }


    pub fn is_open(
        &mut self
    ) -> Result<()> {
        require!(self.result == ProposalStatus::Open, DaoError::InvalidProposalStatus);
        Ok(())
    }

    pub fn is_succeeded(
        &self
    ) -> Result<()> {
        require!(self.result == ProposalStatus::Succeeded, DaoError::InvalidProposalStatus);
        Ok(())
    }

    pub fn is_failed(
        &self
    ) -> Result<()> {
        require!(self.result == ProposalStatus::Failed, DaoError::InvalidProposalStatus);
        Ok(())
    }

    pub fn add_vote(
        &mut self,
        amount: u64,
        choice : VoteChoice,
    ) -> Result<()> {
        self.try_initialize();
        require!(self.result == ProposalStatus::Open, DaoError::InvalidProposalStatus);
        self.votes = self.votes.checked_add(amount).ok_or(DaoError::Overflow)?;
        
    match vote_choice {
        VoteChoice::For => {
            self.vote_counts[0] = self.vote_counts[0].checked_add(amount).ok_or(DaoError::Overflow)?;
        }
        VoteChoice::Against => {
            self.vote_counts[1] = self.vote_counts[1].checked_add(amount).ok_or(DaoError::Overflow)?;
        }
        VoteChoice::Abstain => {
            self.vote_counts[2] = self.vote_counts[2].checked_add(amount).ok_or(DaoError::Overflow)?;
        }
    }
        self.try_finalize();
        Ok(())
    }

    pub fn remove_vote(
        &mut self,
        amount: u64,
        choice : VoteChoice,
    ) -> Result<()> {
        require!(self.result == ProposalStatus::Open, DaoError::InvalidProposalStatus);
        self.votes = self.votes.checked_sub(amount).ok_or(DaoError::Underflow)?;
        match vote_choice {
            VoteChoice::For => {
                self.vote_counts[0] = self.vote_counts[0].checked_sub(amount).ok_or(DaoError::Underflow)?;
            }
            VoteChoice::Against => {
                self.vote_counts[1] = self.vote_counts[1].checked_sub(amount).ok_or(DaoError::Underflow)?;
            }
            VoteChoice::Abstain => {
                self.vote_counts[2] = self.vote_counts[2].checked_sub(amount).ok_or(DaoError::Underflow)?;
            }
        }
        Ok(())
    }


#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum VoteType {
    SingleChoice,
    MultipleChoice,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
pub enum ProposalType {
    Bounty(Pubkey, u64), // Pay an address some amount of SOL
    Executable, // Sign some kind of instruction(s) with an accounts struct, etc
    Vote // We just want to know what people think. No money involved
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ProposalStatus {
    PreVoting,
    Open,
    Succeeded,
    Failed
}