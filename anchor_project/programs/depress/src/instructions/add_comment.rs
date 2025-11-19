//-------------------------------------------------------------------------------
///
/// TASK: Implement the "add comment" functionality for the DePress program
///
/// Requirements:
/// - Enforce that `comment_content` does not exceed the maximum allowed length
/// - Initialize a new `Comment` account as a PDA using seeds:
///   [COMMENT_SEED, parent_post pubkey, author pubkey, SHA256(content)]
/// - Populate comment fields: content, author, parent post, and bump
/// - Initialize engagement counters (likes/dislikes) to zero
/// - Increment the parent post's `comment_count`
/// - Emit a `CommentCreated` event for indexing
///
/// The content hash ensures comment uniqueness per author and post,
/// preventing duplicate submissions while allowing multiple comments
/// from the same user on the same post if content differs.
///
//-------------------------------------------------------------------------------
use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;

use crate::constants::{COMMENT_LENGTH, COMMENT_SEED, DISCRIMINATOR};
use crate::errors::DepressError;
use crate::states::{Comment, Post};

pub fn add_comment(ctx: Context<AddCommentContext>, comment_content: String) -> Result<()> {
    // Validate comment content length in bytes
    require!(
        comment_content.as_bytes().len() <= COMMENT_LENGTH,
        DepressError::CommentTooLong
    );

    // Initialize the comment account
    let comment = &mut ctx.accounts.comment;
    let post = &mut ctx.accounts.post;

    comment.comment_author = ctx.accounts.comment_author.key();
    comment.parent_post = post.key();
    comment.content = comment_content.clone(); // Clone to use in event
    comment.likes = 0;
    comment.dislikes = 0;
    comment.bump = ctx.bumps.comment;

    // Increment comment count on the post
    post.comment_count += 1;

    // Emit event
    emit!(CommentCreated {
        author: comment.comment_author,
        parent_post: post.key(),
        content: comment_content,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(comment_content: String)]
pub struct AddCommentContext<'info> {
    #[account(mut)]
    pub comment_author: Signer<'info>,

    #[account(mut)]
    pub post: Account<'info, Post>,

    #[account(
        init,
        payer = comment_author,
        space = DISCRIMINATOR + Comment::INIT_SPACE,
        seeds = [
            COMMENT_SEED.as_bytes(),
            comment_author.key().as_ref(),
            {hash(comment_content.as_bytes()).to_bytes().as_ref()},
            post.key().as_ref(),
        ],
        bump
    )]
    pub comment: Account<'info, Comment>,

    pub system_program: Program<'info, System>,
}

#[event]
pub struct CommentCreated {
    pub author: Pubkey,
    pub parent_post: Pubkey,
    pub content: String,
}
