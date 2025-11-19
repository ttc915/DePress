/// //-------------------------------------------------------------------------------
///
/// TASK: Implement the "add post" functionality for the decentralized DePress program
///
/// Requirements:
/// - Enforce that `topic` and `content` do not exceed their respective maximum lengths
/// - Initialize a new `Post` account as a PDA using the seeds: [POST_SEED, topic, author pubkey]
/// - Populate the post with the provided topic, content, and author
/// - Initialize engagement counters (likes, dislikes, comments) to zero
/// - Store the PDA bump for future rederivation
///
/// The `Post` account is owned by the program and funded by the post author.
/// The topic is part of the PDA seed to allow multiple posts per user under different topics.
///
//-------------------------------------------------------------------------------
use anchor_lang::prelude::*;

use crate::constants::{CONTENT_LENGTH, DISCRIMINATOR, POST_SEED, TOPIC_LENGTH};
use crate::errors::DepressError;
use crate::states::Post;

pub fn add_post(
    ctx: Context<AddPostContext>,
    post_topic: String,
    post_content: String,
) -> Result<()> {
    // Validate topic and content lengths
    require!(
        post_topic.as_bytes().len() <= TOPIC_LENGTH,
        DepressError::TopicTooLong
    );

    require!(
        post_content.as_bytes().len() <= CONTENT_LENGTH,
        DepressError::ContentTooLong
    );

    // Initialize the post account
    let post = &mut ctx.accounts.post;
    post.post_author = ctx.accounts.post_author.key();

    post.topic = post_topic.clone();
    post.content = post_content;

    post.likes = 0;
    post.dislikes = 0;
    post.comment_count = 0;

    post.bump = ctx.bumps.post;

    // Emit event
    emit!(PostCreated {
        post_author: post.post_author,
        topic: post_topic,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(post_topic: String)]
pub struct AddPostContext<'info> {
    #[account(
        init,
        payer = post_author,
        space = DISCRIMINATOR + Post::INIT_SPACE,
        seeds = [
            POST_SEED.as_bytes(),
            post_topic.as_bytes(),
            post_author.key().as_ref()
        ],
        bump
    )]
    pub post: Account<'info, Post>,

    #[account(mut)]
    pub post_author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct PostCreated {
    pub post_author: Pubkey,
    pub topic: String,
}
