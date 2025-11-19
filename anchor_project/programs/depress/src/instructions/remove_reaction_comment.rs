//-------------------------------------------------------------------------------
///
/// TASK: Implement the "remove reaction from comment" functionality for the DePress program
///
/// Requirements:
/// - Close the existing reaction account and refund rent to the author
/// - Decrement the correct counter (likes/dislikes) on the parent comment
/// - Ensure the author is the reaction owner (enforced via constraints)
/// - Prevent underflow by validating counters are > 0
/// - Emit a `ReactionRemoved` event for indexing
///
/// The reaction type is read from the account — user input is not trusted.
///
//-------------------------------------------------------------------------------
use anchor_lang::prelude::*;

use crate::constants::COMMENT_REACTION_SEED;
use crate::errors::DepressError;
use crate::states::{Comment, ReactionComment, ReactionType};

pub fn remove_reaction_comment(ctx: Context<RemoveReactionCommentContext>) -> Result<()> {
    let comment_reaction = &ctx.accounts.comment_reaction.reaction;
    let comment = &mut ctx.accounts.comment;

    // Safely decrement counter based on stored reaction type
    match comment_reaction {
        ReactionType::Like => {
            require!(comment.likes > 0, DepressError::InvalidReactionState);
            comment.likes -= 1;
        }
        ReactionType::Dislike => {
            require!(comment.dislikes > 0, DepressError::InvalidReactionState);
            comment.dislikes -= 1;
        }
    }

    // Emit event before account is closed
    emit!(ReactionCommentRemoved {
        author: ctx.accounts.reaction_author.key(),
        parent_comment: comment.key(),
        reaction: match comment_reaction {
            ReactionType::Like => 0,
            ReactionType::Dislike => 1,
        },
    });

    Ok(())
}

#[derive(Accounts)]
pub struct RemoveReactionCommentContext<'info> {
    #[account(
        mut,
        has_one = reaction_author @ DepressError::InvalidOwner,
        close = reaction_author,
        seeds = [
            COMMENT_REACTION_SEED.as_bytes(),
            reaction_author.key().as_ref(),
            comment.key().as_ref(),
        ],
        bump = comment_reaction.bump,
    )]
    pub comment_reaction: Account<'info, ReactionComment>,

    #[account(mut)]
    pub comment: Account<'info, Comment>,

    #[account(mut)]
    pub reaction_author: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[event]
pub struct ReactionCommentRemoved {
    pub author: Pubkey,
    pub parent_comment: Pubkey,
    // 0 = Like, 1 = Dislike
    pub reaction: u8,
}
