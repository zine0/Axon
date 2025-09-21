# Axon OJ System

A distributed Online Judge system built with Rust Axum backend, Vue frontend, and multiple judger services.

## Architecture

- **Backend**: Rust Axum API server
- **Frontend**: Vue 3 with TypeScript
- **Judger**: Rust-based distributed judging service
- **Shared**: Common types and protocols

## Development

### Backend
```bash
cd backend
cargo run
```

### Frontend
```bash
cd frontend
pnpm install
pnpm dev
```

### Judger
```bash
cd judger
cargo run
```

## Features
- User authentication
- Problem management
- Code submission and judging
- Real-time results
- Distributed judging system