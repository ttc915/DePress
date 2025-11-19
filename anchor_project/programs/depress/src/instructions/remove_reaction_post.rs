//-------------------------------------------------------------------------------
///
/// TASK: Implement the "remove reaction from post" functionality for the DePress program
///
/// Requirements:
/// - Verify the reaction exists and belongs to the reaction author (enforced via PDA + has_one)
/// - Decrement the appropriate counter (likes/dislikes) on the parent post
/// - Prevent underflow by ensuring counters are > 0
/// - Close the reaction account and refund rent to the author
/// - Emit a `ReactionPostRemoved` event for off-chain indexing
///
/// The PDA constraint and `has_one` ensure only the true author can remove their reaction.
///
//-------------------------------------------------------------------------------
use anchor_lang::prelude::*;

use crate::constants::POST_REACTION_SEED;
use crate::errors::DepressError;
use crate::states::{Post, ReactionPost, ReactionType};

pub fn remove_reaction_post(ctx: Context<RemoveReactionPostContext>) -> Result<()> {
    let reaction = &ctx.accounts.post_reaction;
    let post = &mut ctx.accounts.post;

    // Safely decrement the appropriate counter
    match reaction.reaction {
        ReactionType::Like => {
            require!(post.likes > 0, DepressError::InvalidReactionState);
            post.likes -= 1;
        }
        ReactionType::Dislike => {
            require!(post.dislikes > 0, DepressError::InvalidReactionState);
            post.dislikes -= 1;
        }
    }

    // Emit event before account is closed
    emit!(ReactionPostRemoved {
        author: ctx.accounts.reaction_author.key(),
        parent_post: post.key(),
        reaction: match reaction.reaction {
            ReactionType::Like => 0,
            ReactionType::Dislike => 1,
        },
    });

    Ok(())
}

#[derive(Accounts)]
pub struct RemoveReactionPostContext<'info> {
    #[account(
        mut,
        has_one = reaction_author @ DepressError::InvalidOwner,
        close = reaction_author,
        seeds = [
            POST_REACTION_SEED.as_bytes(),
            reaction_author.key().as_ref(),
            post.key().as_ref(),
        ],
        bump = post_reaction.bump,
    )]
    pub post_reaction: Account<'info, ReactionPost>,

    #[account(mut)]
    pub post: Account<'info, Post>,

    #[account(mut)]
    pub reaction_author: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[event]
pub struct ReactionPostRemoved {
    pub author: Pubkey,
    pub parent_post: Pubkey,
    // 0 = Like, 1 = Dislike
    pub reaction: u8,
}
