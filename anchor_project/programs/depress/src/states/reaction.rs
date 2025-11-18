use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub enum ReactionType {
    Like,
    Dislike,
}

#[account]
#[derive(InitSpace)]
pub struct Reaction {
    pub reaction_author: Pubkey,
    pub parent_post: Pubkey,
    pub reaction: ReactionType,
    pub bump: u8,
}
