//-------------------------------------------------------------------------------
///
/// TASK: Implement the "remove comment" functionality for the DePress program
///
/// Requirements:
/// - Only the comment author may delete the comment
/// - Close the comment account and return rent to the author
/// - Verify PDA is derived using the same seeds as during creation:
///   [COMMENT_SEED, parent_post, author, SHA256(content)]
/// - Emit a `CommentRemoved` event for off-chain indexing
///
/// NOTE: The instruction requires the original `content` to recompute the PDA.
///
//-------------------------------------------------------------------------------
use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;

use crate::constants::COMMENT_SEED;
use crate::states::Comment;

pub fn remove_comment(ctx: Context<RemoveCommentContext>) -> Result<()> {
    let comment = &ctx.accounts.comment;

    // Optional: verify content matches (defense-in-depth)
    // (Not strictly needed due to PDA constraint, but improves event accuracy)
    // require!(comment.content == content, DepressError::InvalidContent);

    emit!(CommentRemoved {
        author: comment.comment_author,
        parent_post: comment.parent_post.key(),
        content: comment.content.clone(),
    });

    Ok(())
}

#[derive(Accounts)]
pub struct RemoveCommentContext<'info> {
    #[account(mut)]
    pub comment_author: Signer<'info>,

    #[account(
        mut,
        has_one = comment_author,
        close = comment_author,
        seeds = [
            COMMENT_SEED.as_bytes(),
            comment_author.key().as_ref(),
            {hash(comment.content.as_bytes()).to_bytes().as_ref()},
            comment.parent_post.as_ref(),
        ],
        bump = comment.bump,
    )]
    pub comment: Account<'info, Comment>,
}

#[event]
pub struct CommentRemoved {
    pub author: Pubkey,
    pub parent_post: Pubkey,
    pub content: String,
}
