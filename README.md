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
dfx start --clean --background
```
```
dfx deploy backend
```
```
dfx deploy frontend
```
```
 or run a local dev server:
cd frontend && npm run dev
```

## Example Interactions
After logging in with II:
- **Register**: set your username & avatar URL.
- **Create Post**: write content and submit.
- **Follow**: use the "Demo Principals" list to follow preloaded users to see their posts in your feed.
- **Like**: click "Like".
- **Comment**: expand "Comments" and add your thoughts (UI scaffolded for viewing; add a form similarly to `onPost` to submit).




