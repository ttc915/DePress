# Project Description

**Deployed Frontend URL:** https://depress.vercel.app/

**Solana Program ID:** 5aQmhcFhVmgtmCdGtffRuMYL9R1WsARAtukzxUttPKKN

## Project Overview

### Description

A decentralized social media platform built on Solana called "DePress" where users can create posts, engage with content through comments, and express their opinions with like/dislike reactions. Each user maintains full ownership and control over their content, with all interactions happening on-chain. The platform demonstrates advanced Solana program development concepts including multiple account types, complex PDA relationships, and comprehensive state management.

### Key Features

- **Create Posts**: Users can publish posts with topics (up to 32 characters) and content (up to 500 characters)
- **Comment on Posts**: Add comments to any post with content up to 100 characters
- **Like/Dislike Reactions**: Express approval or disapproval of posts and comments
- **Content Ownership**: Users have full control over their posts and can remove them at any time
- **Reaction Management**: Users can add and remove their reactions to content
- **Deterministic Addressing**: Program Derived Addresses ensure unique content identification and prevent conflicts

### How to Use the dApp

1. **Connect Wallet** - Connect your Solana wallet to interact with the dApp
2. **Create a Post** - Enter a topic (max 32 chars) and content (max 500 chars), then submit to create your post
3. **View Posts** - Browse all posts from different users on the main feed
4. **Add Comments** - Click on any post to add a comment (max 100 chars)
5. **React to Content** - Use like/dislike buttons on posts and comments to express your opinion
6. **Manage Your Content** - Remove your own posts, comments, or reactions as needed

## Program Architecture

The DePress dApp uses a sophisticated architecture with four main account types (Post, Comment, ReactionPost, ReactionComment) and nine core instructions. The program leverages multiple layers of Program Derived Addresses to create deterministic, conflict-free account relationships. Posts are the root entities, with comments and reactions branching off them. Each user action creates or modifies accounts in a hierarchical structure that maintains data integrity and ownership.

### PDA Usage

The program uses four different PDA patterns to create unique, deterministic addresses for all account types. This ensures that content can be reliably located and prevents account collisions between different users and content.

**PDAs Used:**

- **Post PDA**: Derived from seeds `["POST_SEED", topic, author_pubkey]` - ensures each user can have multiple posts with different topics, but prevents duplicate posts with the same topic from the same author
- **Comment PDA**: Derived from seeds `["COMMENT_SEED", author_pubkey, content_hash, parent_post_pubkey]` - uses SHA256 hash of comment content to ensure uniqueness, allowing users to make multiple comments on the same post with different content
- **Post Reaction PDA**: Derived from seeds `["POST_REACTION_SEED", author_pubkey, post_pubkey]` - ensures each user can have only one reaction per post, preventing duplicate reactions
- **Comment Reaction PDA**: Derived from seeds `["COMMENT_REACTION_SEED", author_pubkey, comment_pubkey]` - ensures each user can have only one reaction per comment, maintaining reaction integrity

### Program Instructions

**Instructions Implemented:**

- **post_add**: Creates a new post account with topic and content, initializes counters to zero
- **post_remove**: Closes a post account and refunds rent, can only be called by the post author
- **like_post**: Creates a reaction account with Like type, increments post's like counter
- **dislike_post**: Creates a reaction account with Dislike type, increments post's dislike counter
- **reaction_remove_post**: Closes a post reaction account, decrements appropriate counter
- **comment_add**: Creates a comment account linked to a parent post, increments post's comment count
- **comment_remove**: Closes a comment account, can only be called by the comment author
- **like_comment**: Creates a comment reaction with Like type, increments comment's like counter
- **dislike_comment**: Creates a comment reaction with Dislike type, increments comment's dislike counter
- **reaction_remove_comment**: Closes a comment reaction account, decrements appropriate counter

### Account Structure

```rust
#[account]
#[derive(InitSpace)]
pub struct Post {
    pub post_author: Pubkey,        // The wallet that created this post
    #[max_len(32)]
    pub topic: String,              // Post topic (max 32 chars)
    #[max_len(500)]
    pub content: String,            // Post content (max 500 chars)
    pub likes: u64,                 // Number of likes
    pub dislikes: u64,              // Number of dislikes
    pub comment_count: u32,         // Number of comments on this post
    pub bump: u8,                   // PDA bump for validation
}

#[account]
#[derive(InitSpace)]
pub struct Comment {
    pub comment_author: Pubkey,     // The wallet that created this comment
    pub parent_post: Pubkey,        // Reference to the post this comments on
    #[max_len(100)]
    pub content: String,            // Comment content (max 100 chars)
    pub likes: u64,                 // Number of likes on this comment
    pub dislikes: u64,              // Number of dislikes on this comment
    pub bump: u8,                   // PDA bump for validation
}

#[account]
#[derive(InitSpace)]
pub struct ReactionPost {
    pub reaction_author: Pubkey,    // The wallet that created this reaction
    pub parent_post: Pubkey,        // Reference to the post being reacted to
    pub reaction: ReactionType,     // Like or Dislike enum
    pub bump: u8,                   // PDA bump for validation
}

#[account]
#[derive(InitSpace)]
pub struct ReactionComment {
    pub reaction_author: Pubkey,    // The wallet that created this reaction
    pub parent_comment: Pubkey,     // Reference to the comment being reacted to
    pub reaction: ReactionType,     // Like or Dislike enum
    pub bump: u8,                   // PDA bump for validation
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub enum ReactionType {
    Like,
    Dislike,
}
```

## Testing

### Test Coverage

Comprehensive test suite covering all 10 instructions with both successful operations and error conditions. Tests include boundary testing for content lengths, authorization checks, and edge cases with unicode characters and emojis.

**Happy Path Tests:**

- **Post Creation**: Successfully creates posts with various content lengths, topics, and unicode characters
- **Comment Creation**: Successfully adds comments to posts with proper linking and counter updates
- **Reaction Management**: Successfully adds and removes like/dislike reactions on posts and comments
- **Content Removal**: Successfully removes posts, comments, and reactions with proper account closure
- **Multiple Users**: Successfully handles interactions between different users on the same content

**Unhappy Path Tests:**

- **Content Length Limits**: Fails when post content exceeds 500 chars, topics exceed 32 chars, or comments exceed 100 chars
- **Duplicate Content**: Fails when creating duplicate posts with same topic/author or duplicate reactions
- **Authorization**: Fails when non-owners attempt to remove posts, comments, or reactions
- **Non-existent Content**: Fails when reacting to or commenting on posts/comments that don't exist
- **Account Validation**: Fails with proper constraint errors when account seeds don't match expected PDAs

### Running Tests

```bash
# Install dependencies (if needed)
pnpm install

# Run tests
pnpm test
```

### Additional Notes for Evaluators

This project demonstrates advanced Solana development concepts including complex multi-account relationships, hierarchical PDA structures, and comprehensive error handling. The biggest challenges were managing the interdependencies between posts, comments, and reactions while ensuring proper authorization and state consistency. The use of SHA256 hashing for comment PDAs was particularly interesting for ensuring content uniqueness while allowing multiple comments per user per post. The project includes both a fully functional Solana program and a Next.js frontend with proper wallet integration.
