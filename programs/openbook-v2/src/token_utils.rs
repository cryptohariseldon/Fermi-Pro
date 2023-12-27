use super::*;
use anchor_lang::system_program;
use anchor_spl::token;
use anchor_spl::token::Approve;
use anchor_spl::token::Transfer;

use error::OpenBookError;

pub fn token_transfer<
    'info,
    P: ToAccountInfo<'info>,
    A: ToAccountInfo<'info>,
    S: ToAccountInfo<'info>,
>(
    amount: u64,
    token_program: &P,
    from: &A,
    to: &A,
    authority: &S,
) -> Result<()> {
    if amount > 0 {
        token::transfer(
            CpiContext::new(
                token_program.to_account_info(),
                token::Transfer {
                    from: from.to_account_info(),
                    to: to.to_account_info(),
                    authority: authority.to_account_info(),
                },
            ),
            amount,
        )
    } else {
        Ok(())
    }
}

pub fn token_transfer_signed<
    'info,
    P: ToAccountInfo<'info>,
    A: ToAccountInfo<'info>,
    L: ToAccountInfo<'info>,
>(
    amount: u64,
    token_program: &P,
    from: &A,
    to: &A,
    authority: &L,
    seeds: &[&[u8]],
) -> Result<()> {
    if amount > 0 {
        token::transfer(
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                token::Transfer {
                    from: from.to_account_info(),
                    to: to.to_account_info(),
                    authority: authority.to_account_info(),
                },
                &[seeds],
            ),
            amount,
        )
    } else {
        Ok(())
    }
}

pub fn system_program_transfer<
    'info,
    S: ToAccountInfo<'info>,
    A: ToAccountInfo<'info>,
    L: ToAccountInfo<'info>,
>(
    amount: u64,
    system_program: &S,
    from: &A,
    to: &L,
) -> Result<()> {
    if amount > 0 {
        system_program::transfer(
            CpiContext::new(
                system_program.to_account_info(),
                system_program::Transfer {
                    from: from.to_account_info(),
                    to: to.to_account_info(),
                },
            ),
            amount,
        )
    } else {
        Ok(())
    }
}

    pub fn token_approve<
    'info,
    P: ToAccountInfo<'info>,
    T: ToAccountInfo<'info>,
    D: ToAccountInfo<'info>,
    A: ToAccountInfo<'info>,
>(
    amount: u64,
    token_program: &P,
    token_account: &T,
    delegate: &D,
    authority: &A,
) -> Result<()> {
    if amount > 0 {
        let approve_instruction = Approve {
            to: token_account.to_account_info(),
            delegate: delegate.to_account_info(),
            authority: authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(token_program.to_account_info(), approve_instruction);
        token::approve(cpi_ctx, amount).map_err(|err| match err {
            _ => error!(OpenBookError::ApprovalFailed),
        })?;
        msg!("Tokens approved for later spending.");
    }
    Ok(())
}

    pub fn token_approve2<
    'info,
    P: ToAccountInfo<'info>,
    A: ToAccountInfo<'info>,
    D: ToAccountInfo<'info>, // Delegate account
    O: ToAccountInfo<'info>, // Owner account
>(
    amount: u64,
    token_program: &P,
    token_account: &A,
    delegate: &D,
    owner: &O,
) -> Result<()> {
    if amount > 0 {
        token::approve(
            CpiContext::new(
                token_program.to_account_info(),
                Approve {
                    to: token_account.to_account_info(),
                    delegate: delegate.to_account_info(),
                    authority: owner.to_account_info(),
                },
            ),
            amount,
        )
    } else {
        Ok(())
    }
}

/// Transfers tokens from a user's account to another account using the market's PDA as authority.
pub fn transfer_from_user<
    'info,
    P: ToAccountInfo<'info>,
    A: ToAccountInfo<'info>,
    M: ToAccountInfo<'info>,
>(
    amount: u64,
    token_program: &P,
    from: &A,
    to: &A,
    market: &M,
    seeds: &[&[u8]],
) -> Result<()> {
    if amount > 0 {
        let transfer_ix = Transfer {
            from: from.to_account_info(),
            to: to.to_account_info(),
            authority: market.to_account_info(), // Using the market PDA as the authority.
        };

        // Correctly format the seeds as a slice of slices of slices of bytes
        let seeds_slices: &[&[u8]] = seeds;
        let signer_seeds: &[&[&[u8]]] = &[seeds_slices];

        let cpi_ctx = CpiContext::new_with_signer(
            token_program.to_account_info(),
            transfer_ix,
            signer_seeds,
        );

        token::transfer(cpi_ctx, amount).map_err(|err| match err {
            _ => error!(OpenBookError::ApprovalFailed),
        })?;
    }
    Ok(())
}


