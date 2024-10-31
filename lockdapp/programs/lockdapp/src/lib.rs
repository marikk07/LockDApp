// use anchor_lang::prelude::*;
//
// declare_id!("6i8FHTnCRhGC3XEH2FKwtKJ4zeEvtKPckcUUXLDz69ty");
//
// #[program]
// pub mod lockdapp {
//     use super::*;
//
//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         Ok(())
//     }
// }
//
// #[derive(Accounts)]
// pub struct Initialize {}

use anchor_lang::prelude::*;
use sha2::{Sha256, Digest};

declare_id!("6i8FHTnCRhGC3XEH2FKwtKJ4zeEvtKPckcUUXLDz69tu");

#[program]
mod escrow_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64, condition: String) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;

        // Hash the condition string
        let mut hasher = Sha256::new();
        hasher.update(condition.as_bytes());
        escrow.condition_hash = hasher.finalize().into();

        // Store the authority and amount
        escrow.authority = *ctx.accounts.authority.key;
        escrow.amount = amount;
        escrow.is_released = false;

        msg!("Escrow initialized with condition hash: {:?}", escrow.condition_hash);

        Ok(())
    }

    pub fn release(ctx: Context<Release>, input: String) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;

        // Hash the input and compare it to the stored hash
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let input_hash = hasher.finalize();

        if input_hash.as_slice() != escrow.condition_hash {
            return Err(EscrowError::IncorrectCondition.into());
        }

        if escrow.is_released {
            return Err(EscrowError::FundsAlreadyReleased.into());
        }

        // Release the funds
        escrow.is_released = true;
        msg!("Funds released!");

        Ok(())
    }
}

// Escrow account structure
#[account]
pub struct Escrow {
    pub authority: Pubkey,
    pub amount: u64,
    pub condition_hash: [u8; 32],  // Store the hash instead of the plain string
    pub is_released: bool,
    pub bump: u8,
}

// Custom errors
#[error_code]
pub enum EscrowError {
    #[msg("Incorrect condition provided.")]
    IncorrectCondition,
    #[msg("Funds have already been released.")]
    FundsAlreadyReleased,
}

// Context for initialize function
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 32 + 8 + 1 + 1)]
    pub escrow: Account<'info, Escrow>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Context for release function
#[derive(Accounts)]
pub struct Release<'info> {
    #[account(mut, has_one = authority)]
    pub escrow: Account<'info, Escrow>,
    pub authority: Signer<'info>,
}

