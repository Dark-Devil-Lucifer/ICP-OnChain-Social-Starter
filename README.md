# ICP On-Chain Social (Rust + React + Vite)

A fully on-chain social starter on the Internet Computer. All posts, profiles, follows, likes, and comments live in the Rust canister state with upgrade-safe persistence. Internet Identity is used for login. The frontend is a Vite + React app deployed as an asset canister.

## Features
- 100% on-chain data model (profiles, posts, follows, likes, comments).
- Rust canister with stable (pre/post-upgrade) persistence.
- Type-safe Candid interface (included `backend.did` and `did.js` for the frontend).
- Internet Identity authentication (uses the hosted II: `identity.ic0.app`).
- Real-time-ish updates with polling query calls.
- Demo data preloaded for 5 users (alice, bob, charlie, diana, eve) each with posts, likes, comments, and follow graphs.

## Prerequisites
- Rust with wasm target: `rustup target add wasm32-unknown-unknown`
- DFX: https://internetcomputer.org/docs/current/developer-docs/setup/install/
- Node.js >= 18

## Quick Start (Local Dev)
```bash
# 1) Start the local replica
dfx start --clean --background

# 2) Deploy the backend canister
dfx deploy backend

# 3) Deploy the frontend (builds Vite and uploads assets canister)
dfx deploy frontend

# 4) Open the frontend canister URL printed by the previous command
#    or run a local dev server:
cd frontend && npm run dev
# For dev server, create .env.local and set:
# VITE_BACKEND_CANISTER_ID=<canister id from `dfx canister id backend`>
```

Tip: Retrieve the backend canister id anytime:
```bash
dfx canister id backend
```

## Internet Identity
The app uses the hosted II at `https://identity.ic0.app/#authorize` and works from localhost during development.

## Example Interactions
After logging in with II:
- **Register**: set your username & avatar URL.
- **Create Post**: write content and submit.
- **Follow**: use the "Demo Principals" list to follow preloaded users to see their posts in your feed.
- **Like**: click "Like".
- **Comment**: expand "Comments" and add your thoughts (UI scaffolded for viewing; add a form similarly to `onPost` to submit).

## Key Backend Methods (Rust)
- `register(username, avatar_url) -> Result<Profile, String>`
- `create_post(content) -> Result<Post, String>`
- `get_my_feed(offset, limit) -> Vec<Post>`
- `follow(user_principal) / unfollow(user_principal)`
- `like_post(post_id) / unlike_post(post_id)`
- `comment_post(post_id, content)`

See `backend/src/lib.rs` for full logic and comments.

## Production Build & Deploy (ICP Mainnet)
- Configure your `identity` and wallet on ICP.
- Deploy with `--network ic`:
```bash
DFX_NETWORK=ic dfx deploy backend --network ic
DFX_NETWORK=ic dfx deploy frontend --network ic
```
- The `frontend` build step uses the environment variable `CANISTER_ID_BACKEND` (exported by `dfx deploy`) to inject the backend canister id at build time.

## Type-Safe Interfaces
- `backend/backend.did` defines the Candid interface.
- Frontend uses `src/declarations/backend.did.js` which mirrors the interface for an `Actor` via `@dfinity/agent`.

## Project Structure
```
icp-onchain-social/
├── dfx.json
├── backend/
│   ├── Cargo.toml
│   ├── backend.did
│   └── src/
│       └── lib.rs
└── frontend/
    ├── index.html
    ├── package.json
    ├── tsconfig.json
    ├── vite.config.ts
    └── src/
        ├── App.tsx
        ├── main.tsx
        ├── agent.ts
        └── declarations/
            └── backend.did.js
```

## Notes on Decentralization
- All state is stored on-chain in the Rust canister. No external DB or server is used.
- For scale, you can shard state across multiple canisters (e.g., per-user or per-post-bucket) and use cross-canister calls while keeping the same interface shape.
- Consider `ic-stable-structures` for very large data sets.

## Security & Validation
- Author-only edit/delete enforced by comparing `caller()` with `post.author`.
- Registration required before creating social actions (ensured by `ensure_profile`).

## Extending
- Add pagination and cursors to feed endpoints.
- Implement a subscriptions pattern with long-polling or signals.
- Add media uploads via separate asset canisters with hashed references stored in posts.
- Index by hashtags/topics as additional maps.

Happy hacking!# ICP-OnChain-Social-Starter
# ICP-OnChain-Social-Starter
# ICP-OnChain-Social-Starter
