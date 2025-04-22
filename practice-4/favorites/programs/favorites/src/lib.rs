use anchor_lang::prelude::*;

declare_id!("Cq3qUjMNF7NdjZy9kmCuhGUM5PnjcAfGHERB9pmTK9KD");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

// What we will put inside the Favorites PDA
#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,

    #[max_len(50)]
    pub color: String,

    pub delegate: Option<Pubkey>,
}

// When people call the set_favorites instruction, they will need to provide the accounts that will
// be modified. This keeps Solana fast!
#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump,
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetAuthority<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"favorites", user.key().as_ref()],
        bump,
    )]
    pub favorites: Account<'info, Favorites>,
}

#[derive(Accounts)]
pub struct SetFavoritesWithDelegate<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump,
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"favorites", original_owner.key().as_ref()],
        bump,
        constraint = user.key() == original_owner.key() || matches!(favorites.delegate, Some(delegate_key) if user.key() == delegate_key)
    )]
    pub favorites: Account<'info, Favorites>,
    
    /// CHECK: The original_owner is validated by the seeds constraint and the constraint logic.
    pub original_owner: AccountInfo<'info>,
}

// Our Solana program!
#[program]
pub mod favorites {
    use super::*;

    // Our instruction handler! It sets the user's favorite number and color
    pub fn set_favorites(context: Context<SetFavorites>, number: u64, color: String) -> Result<()> {
        let user_public_key = context.accounts.user.key();
        msg!("Greetings from {}", context.program_id);
        msg!(
            "User {}'s favorite number is {} and favorite color is: {}",
            user_public_key,
            number,
            color
        );

        context.accounts.favorites.set_inner(Favorites { 
            number, 
            color, 
            delegate: None 
        });
        
        Ok(())
    }

    pub fn set_authority(
        context: Context<SetAuthority>, 
        delegate: Option<Pubkey>
    ) -> Result<()> {
        let user_public_key = context.accounts.user.key();
        let favorites = &mut context.accounts.favorites;
        
        match delegate {
            Some(delegate_key) => {
                msg!("Setting delegate for user {} to {}", user_public_key, delegate_key);
                favorites.delegate = Some(delegate_key);
            },
            None => {
                msg!("Removing delegate for user {}", user_public_key);
                favorites.delegate = None;
            }
        }
        
        Ok(())
    }

    pub fn set_favorites_with_delegate(
        context: Context<SetFavoritesWithDelegate>, 
        number: u64, 
        color: String,
        delegate: Option<Pubkey>
    ) -> Result<()> {
        let user_public_key = context.accounts.user.key();
        msg!("Greetings from {}", context.program_id);
        msg!(
            "User {}'s favorite number is {} and favorite color is: {}",
            user_public_key,
            number,
            color
        );
        
        if let Some(delegate_key) = delegate {
            msg!("Delegate set to: {}", delegate_key);
        }
    
        context.accounts.favorites.set_inner(Favorites { 
            number, 
            color, 
            delegate 
        });
        
        Ok(())
    }

    pub fn update_favorites(
        context: Context<UpdateFavorites>, 
        number: Option<u64>, 
        color: Option<String>
    ) -> Result<()> {
        let user_public_key = context.accounts.user.key();
        msg!("Updating favorites for user {}", user_public_key);
        
        let favorites = &mut context.accounts.favorites;
        
        // Update number
        if let Some(new_number) = number {
            msg!("Updated favorite number to {}", new_number);
            favorites.number = new_number;
        }
        
        // Update color
        if let Some(new_color) = color {
            msg!("Updated favorite color to {}", new_color);
            favorites.color = new_color;
        }
        
        Ok(())
    }
}
