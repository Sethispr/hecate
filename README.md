# uptime.rs Setup Guide

To run this project locally, you need to start both the modern Svelte frontend and the lean, modular Rust backend powered by Axum.

## 1. Rust Backend (Axum)

The backend handles the health pooling and connects to target services, specifically designed to monitor `sumi` (https://github.com/blairtcg/sumi). It serves the health metrics on port `3001`.

The Rust codebase is highly modular and optimally structured:

- `main.rs`: Entry point and server configuration
- `health.rs`: Health checkers that fetch `/health` and detailed `/metrics` from Sumi
- `routes.rs`: Handlers for API endpoints connecting the data

**Prerequisites:**

- Ensure you have Rust installed via `rustup` (https://rustup.rs/)
- Optionally, have a local instance of `sumi` running on port `8080` to see real metrics being pulled!

**Running:**

```bash
cd backend
cargo run
```

## 2. Svelte Frontend (Vite)

The Svelte frontend monitors port `3001` for the backend. If the backend is running, it will automatically populate the service statuses in real-time, including rendering detailed cache, requests, and uptime stats right from `sumi`.

**Prerequisites:**

- Node.js installed

**Running:**

```bash
npm install
npm run dev
```

The frontend will be available at `http://localhost:5173`.
