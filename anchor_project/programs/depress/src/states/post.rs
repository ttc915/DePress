use crate::constants::{CONTENT_LENGTH, TOPIC_LENGTH};
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Post {
    pub post_author: Pubkey,
    #[max_len(TOPIC_LENGTH)]
    pub topic: String,
    #[max_len(CONTENT_LENGTH)]
    pub content: String,
    pub likes: u64,
    pub dislikes: u64,
    pub comment_count: u32,
    pub bump: u8,
}
