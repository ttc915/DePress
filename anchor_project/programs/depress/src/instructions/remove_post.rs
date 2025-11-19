//-------------------------------------------------------------------------------
///
/// TASK: Implement the "remove post" functionality for the decentralized DePress program
///
/// Requirements:
/// - Only the original post author may delete the post
/// - Close the `Post` account and return its lamports to the author
/// - Verify the PDA is derived using the correct seeds: [POST_SEED, topic, author pubkey]
/// - Emit a `PostRemoved` event for off-chain indexing
///
/// The instruction requires the exact `topic` used during post creation to ensure
/// correct PDA derivation. Account closure is handled automatically via the `close`
/// constraint, which also enforces ownership and refunds rent.
///
//-------------------------------------------------------------------------------
use anchor_lang::prelude::*;

use crate::constants::POST_SEED;
use crate::states::Post;

pub fn remove_post(ctx: Context<RemovePostContext>) -> Result<()> {
    // The account will be automatically closed due to the `close` constraint

    let post = &ctx.accounts.post;
    let topic = post.topic.clone();

    // Emit event
    emit!(PostRemoved {
        post_author: post.post_author,
        topic: topic,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct RemovePostContext<'info> {
    #[account(
        mut,
        has_one = post_author,
        close = post_author,

        seeds = [
            POST_SEED.as_bytes(),
            post.topic.as_bytes(),
            post_author.key().as_ref(),
        ],
        bump = post.bump,
    )]
    pub post: Account<'info, Post>,

    #[account(mut)]
    pub post_author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct PostRemoved {
    pub post_author: Pubkey,
    pub topic: String,
}
