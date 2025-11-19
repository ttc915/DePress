use anchor_lang::prelude::*;

#[error_code]
pub enum DepressError {
    #[msg("Cannot initialize, topic too long")]
    TopicTooLong,
    #[msg("Cannot initialize, content too long")]
    ContentTooLong,
    #[msg("Invalid reaction state")]
    InvalidReactionState,
    #[msg("Maximum number of Likes Reached")]
    MaxLikesReached,
    #[msg("Maximum number of Dislikes Reached")]
    MaxDislikesReached,
    #[msg("Minimum number of Likes Reached")]
    MinLikesReached,
    #[msg("Minimum number of Dislikes Reached")]
    MinDislikesReached,
    #[msg("Invalid owner")]
    InvalidOwner,
    #[msg("Comment too Long")]
    CommentTooLong,
    #[msg("Maximum number of comments reached")]
    TooManyComments,
}
