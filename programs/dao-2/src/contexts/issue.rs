use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{Metadata, MetadataAccount, MasterEditionAccount, CreateMetadataAccountsV3},
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use crate::state::DaoConfig;

#[derive(Accounts)]
pub struct IssueTokens<'info> {
    #[account(mut)]
    initializer: Signer<'info>,
    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint,
        associated_token::authority = initializer
    )]
    initializer_ata: Account<'info, TokenAccount>,
    #[account(
        seeds=[b"auth", config.key().as_ref()],
        bump = config.auth_bump
    )]
    ///CHECK: This is safe. It's just used to sign things
    auth: UncheckedAccount<'info>,
    #[account(
        seeds=[b"treasury", config.key().as_ref()],
        bump = config.treasury_bump
    )]
    treasury: SystemAccount<'info>,
    #[account(
        mut,
        seeds=[b"mint", config.key().as_ref()],
        bump = config.mint_bump
    )]
    mint: Account<'info, Mint>,
    #[account(
        seeds=[b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    config: Account<'info, DaoConfig>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct IssueNft<'info> {
    #[account(mut)]
    initializer: Signer<'info>,
    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint,
        associated_token::authority = initializer
    )]
    initializer_ata: Account<'info, TokenAccount>,
    #[account(init_if_needed,
        seeds=[b"auth", config.key().as_ref()],
        bump = config.auth_bump
    )]
    ///CHECK: This is safe. It's just used to sign things
    auth: UncheckedAccount<'info>,
    #[account(
        seeds=[b"treasury", config.key().as_ref()],
        bump = config.treasury_bump
    )]
    treasury: SystemAccount<'info>,
    #[account(
        init, 
        mint::decimals = 0,
        mint::authority = auth
    )]
    nft_mint: Account<'info, Mint>,
    #[account(
        seeds=[b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    config: Account<'info, DaoConfig>,
    #[account(
        seeds = [b"metadata", nft_mint.key().as_ref(), metadata_program.key().as_ref()]
    )]
    pub nft_metadata: Account<'info, MetadataAccount>,
    #[account(
        seeds = [b"metadata", nft_mint.key().as_ref(), metadata_program.key().as_ref(), b"edition"]
    )]
    pub nft_master: Account<'info, MasterEditionAccount>,
    token_program: Program<'info, Token>,
    metadata_program: Program<'info, Metadata>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

impl<'info> IssueTokens<'info> {
    pub fn deposit_sol(&self) -> Result<()> {
        let accounts = Transfer {
            from: self.initializer.to_account_info(),
            to: self.treasury.to_account_info(),
        };

        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);

        transfer(ctx, self.config.issue_price)
    }

    pub fn issue_tokens(&self) -> Result<()> {
        let accounts = MintTo {
            mint: self.mint.to_account_info(),
            to: self.initializer_ata.to_account_info(),
            authority: self.auth.to_account_info(),
        };

        let seeds = &[
            &b"auth"[..],
            &self.config.key().to_bytes()[..],
            &[self.config.auth_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        mint_to(ctx, self.config.issue_price)
    }

    pub fn issue_nft(&self) -> Result<()> {
        let accounts = MintTo {
            mint: self.nft_mint.to_account_info(),
            to: self.initializer_ata.to_account_info(),
            authority: self.auth.to_account_info(),
        };

        let seeds = &[
            &b"auth"[..],
            &self.config.key().to_bytes()[..],
            &[self.config.auth_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        mint_to(ctx, 1u64);

        let meta_data_cpi = CreateMetadataAccountsV3 {
            metadata: ctx.accounts.metadata.to_account_info(),
            mint: nft_mint.to_account_info(),
            mint_authority: auth.to_account_info(),
            payer: initializer.to_account_info(),
            update_authority: auth.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };

        let meta_ctx = CpiContext::new(ctx.accounts.metadata_program.to_account_info(), meta_data_cpi);

        let data = DataV2 {
            name: "DAOist Voter".to_string(),
            symbol: "vDAO".to_string(),
            uri: "github".to_string(), //this points to a metadata JSON file.
            seller_fee_basis_points: 100, //10000 = 100%; better to set this in DAO config to allow voting
            creators: [{
                address: dao_treasury.to_account_info().key(),
                verified: true,
                share: 100,
            }],
            collection: None, //collection NFT goes here
            uses: None,
        };
    
        let meta_out = create_metadata_accounts_v3(cpi, data, true, true, None)?;

        ///create master edition account.
        /// 
        let master_edition_cpi = CreateMasterEditionV3 {
            metadata: ctx.accounts.metadata.to_account_info(),
            edition: ctx.accounts.master_edition.to_account_info(),
            mint: nft_mint.to_account_info(),
            mint_authority: auth.to_account_info(),
            payer: initializer.to_account_info(),
            update_authority: auth.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        };
        let cpi = CpiContext::new(ctx.accounts.metadata_program.to_account_info(), meta_data_cpi);
    
    
        anchor_spl::metadata::create_master_edition_v3(master_edition_cpi, Some(1))?;

    }
}