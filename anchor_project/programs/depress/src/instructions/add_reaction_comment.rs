//-------------------------------------------------------------------------------
///
/// TASK: Implement the "add reaction to comment" functionality for the DePress program
///
/// Requirements:
/// - Initialize a new `ReactionComment` account as a PDA using seeds:
///   [COMMENT_REACTION_SEED, author pubkey, parent comment pubkey]
/// - Set reaction fields: type (Like/Dislike), author, parent comment, and bump
/// - Increment the corresponding counter (`likes` or `dislikes`) on the parent comment
/// - Prevent duplicate reactions via PDA uniqueness (one reaction per user per comment)
/// - Emit a `ReactionAdded` event for indexing and real-time updates
///
/// Note: The PDA design ensures a user can only react once per comment.
/// To support toggling or updating reactions, additional logic would be needed.
///
//-------------------------------------------------------------------------------
use anchor_lang::prelude::*;

use crate::constants::{COMMENT_REACTION_SEED, DISCRIMINATOR};
use crate::states::{Comment, ReactionComment, ReactionType};

pub fn add_reaction_comment(
    ctx: Context<AddReactionCommentContext>,
    reaction: ReactionType,
) -> Result<()> {
    let comment_reaction = &mut ctx.accounts.comment_reaction;
    let comment = &mut ctx.accounts.comment;
    let reaction_author = ctx.accounts.reaction_author.key();
    let reaction_clone = reaction.clone();

    // Initialize reaction account
    comment_reaction.reaction_author = reaction_author;
    comment_reaction.parent_comment = comment.key();
    comment_reaction.reaction = reaction;
    comment_reaction.bump = ctx.bumps.comment_reaction;

    // Update counters on the parent comment
    match reaction_clone {
        ReactionType::Like => comment.likes += 1,
        ReactionType::Dislike => comment.dislikes += 1,
    }

    // Emit event
    emit!(ReactionCommentAdded {
        author: reaction_author,
        parent_comment: comment.key(),
        reaction: match reaction_clone {
            ReactionType::Like => 0,
            ReactionType::Dislike => 1,
        },
    });

    Ok(())
}

#[derive(Accounts)]
pub struct AddReactionCommentContext<'info> {
    #[account(
        init,
        payer = reaction_author,
        space = DISCRIMINATOR + ReactionComment::INIT_SPACE,
        seeds = [
            COMMENT_REACTION_SEED.as_bytes(),
            reaction_author.key().as_ref(),
            comment.key().as_ref(),
        ],
        bump
    )]
    pub comment_reaction: Account<'info, ReactionComment>,

    #[account(mut)]
    pub comment: Account<'info, Comment>,

    #[account(mut)]
    pub reaction_author: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[event]
pub struct ReactionCommentAdded {
    pub author: Pubkey,
    pub parent_comment: Pubkey,
    // 0 = Like, 1 = Dislike
    pub reaction: u8,
}
