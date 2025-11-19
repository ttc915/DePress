//-------------------------------------------------------------------------------
///
/// TASK: Implement the "add reaction to post" functionality for the DePress program
///
/// Requirements:
/// - Initialize a new `ReactionPost` account as a PDA using seeds:
///   [POST_REACTION_SEED, author pubkey, parent post pubkey]
/// - Set reaction fields: type (Like/Dislike), author, parent post, and bump
/// - Increment the corresponding counter (`likes` or `dislikes`) on the parent post
/// - Prevent duplicate reactions via PDA uniqueness (one per user per post)
/// - Emit a `ReactionPostAdded` event for off-chain indexing
///
/// Note: The PDA design ensures a user can only react once. To support toggling,
/// a separate remove/toggle instruction is needed.
///
//-------------------------------------------------------------------------------
use anchor_lang::prelude::*;

use crate::constants::{DISCRIMINATOR, POST_REACTION_SEED};
use crate::states::{Post, ReactionPost, ReactionType};

pub fn add_reaction_post(
    ctx: Context<AddReactionPostContext>,
    reaction: ReactionType,
) -> Result<()> {
    // Clone the reaction to use it in the match later
    let post = &mut ctx.accounts.post;
    let reaction_author = ctx.accounts.reaction_author.key();
    let reaction_clone = reaction.clone();

    // Initialize the reaction account
    let post_reaction = &mut ctx.accounts.post_reaction;
    post_reaction.reaction_author = reaction_author;
    post_reaction.parent_post = post.key();
    post_reaction.reaction = reaction;
    post_reaction.bump = ctx.bumps.post_reaction;

    // Update the post's like/dislike count

    match reaction_clone {
        ReactionType::Like => post.likes += 1,
        ReactionType::Dislike => post.dislikes += 1,
    }

    // Emit event
    emit!(ReactionPostAdded {
        author: ctx.accounts.reaction_author.key(),
        parent_post: ctx.accounts.post.key(),
        reaction: match reaction_clone {
            ReactionType::Like => 0,
            ReactionType::Dislike => 1,
        },
    });
    Ok(())
}

#[derive(Accounts)]
pub struct AddReactionPostContext<'info> {
    #[account(
        init,
        payer = reaction_author,
        space = DISCRIMINATOR + ReactionPost::INIT_SPACE,
        seeds = [
            POST_REACTION_SEED.as_bytes(),
            reaction_author.key().as_ref(),
            post.key().as_ref()
        ],
        bump
    )]
    pub post_reaction: Account<'info, ReactionPost>,

    #[account(mut)]
    pub post: Account<'info, Post>,

    #[account(mut)]
    pub reaction_author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct ReactionPostAdded {
    pub author: Pubkey,
    pub parent_post: Pubkey,
    // 0 = Like, 1 = Dislike
    pub reaction: u8,
}
