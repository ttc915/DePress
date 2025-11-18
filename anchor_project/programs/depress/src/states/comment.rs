use crate::constants::COMMENT_LENGTH;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Comment {
    pub comment_author: Pubkey,
    pub parent_post: Pubkey,
    #[max_len(COMMENT_LENGTH)]
    pub content: String,
    pub bump: u8,
}
