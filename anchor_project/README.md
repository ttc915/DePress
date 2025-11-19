# DePress - Decentralized Social Media on Solana

DePress is a decentralized social media platform built on the Solana blockchain using the Anchor framework. It allows users to create posts, add comments, and interact through likes and dislikes, all while maintaining ownership and control of their content.

## Features

- **Decentralized Posts**: Create and manage your posts on the Solana blockchain
- **Comment System**: Engage with other users by commenting on posts
- **Reactions**: Express your opinion with like/dislike reactions on both posts and comments
- **Immutable Content**: All content is stored on-chain for transparency and permanence
- **User-Owned**: Users maintain ownership of their content and interactions

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor](https://www.anchor-lang.com/docs/installation)
- [Node.js](https://nodejs.org/) (v16 or later)
- [pnpm](https://pnpm.io/) (recommended), [Yarn](https://yarnpkg.com/), or [npm](https://www.npmjs.com/)

## Getting Started

### 1. Clone the Repository

```bash
git clone <repository-url>
cd depress/anchor_project
```

### 2. Install Dependencies

```bash
# Install JavaScript/TypeScript dependencies (using pnpm - recommended)
pnpm install

# Alternatively, use one of these:
# yarn install
# npm install

# Install Rust dependencies
cargo update
```

### 3. Build the Program

```bash
anchor build
```

### 4. Configure Solana CLI

```bash
# Set to devnet
solana config set --url devnet

# Or local validator
solana config set --url localhost

# Check your configuration
solana config get
```

### 5. Run Tests

```bash
# Run all tests
anchor test

# Run specific test file
anchor test --test depress
```

## Program Architecture

### Key Components

- **Post**: Core content unit with topic and content
- **Comment**: User responses to posts
- **Reaction**: Like/Dislike interactions on posts and comments
- **User**: Wallet-based identity system

### Program Instructions

- `post_add`: Create a new post
- `post_remove`: Remove an existing post
- `like_post`: Like a post
- `dislike_post`: Dislike a post
- `reaction_remove_post`: Remove a reaction from a post
- `comment_add`: Add a comment to a post
- `comment_remove`: Remove a comment
- `like_comment`: Like a comment
- `dislike_comment`: Dislike a comment
- `reaction_remove_comment`: Remove a reaction from a comment

## Smart Contract Details

### Data Structures

- **Post**: Contains topic, content, author, and reaction counts
- **Comment**: Contains content, author, and parent post reference
- **Reaction**: Tracks user reactions to posts and comments

### Security

- All operations are permissioned to the respective content owners
- Input validation to prevent malformed data
- Proper account ownership verification

## Testing

The test suite includes comprehensive tests for all program features:

- Post creation and removal
- Comment functionality
- Reaction management (likes/dislikes)
- Edge cases and error handling
- Access control

To run the test suite:

```bash
anchor test
```

## Deployment

### Local Development

1. Start a local validator:

   ```bash
   solana-test-validator
   ```

2. Deploy the program:
   ```bash
   anchor deploy
   ```

### Devnet/Testnet

1. Configure your wallet:

   ```bash
   solana config set --url devnet
   solana airdrop 1  # Get some test SOL
   ```

2. Build and deploy:
   ```bash
   anchor build
   anchor deploy
   ```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Anchor](https://www.anchor-lang.com/)
- Powered by [Solana](https://solana.com/)
- Inspired by decentralized social media platforms

## Support

For support, please open an issue in the GitHub repository.

## Troubleshooting

If you encounter any issues:

1. **Build errors**: Run `anchor build` to rebuild the program
2. **Test failures**: Make sure the local validator is running with `solana-test-validator`
3. **Deployment issues**: Check your Solana CLI configuration with `solana config get`
4. **Dependency issues**: Run `pnpm install` to ensure all dependencies are installed
