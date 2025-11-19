use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub enum ReactionType {
    Like,
    Dislike,
}

#[account]
#[derive(InitSpace)]
pub struct ReactionPost {
    pub reaction_author: Pubkey,
    pub parent_post: Pubkey,
    pub reaction: ReactionType,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct ReactionComment {
    pub reaction_author: Pubkey,
    pub parent_comment: Pubkey,
    pub reaction: ReactionType,
    pub bump: u8,
}
