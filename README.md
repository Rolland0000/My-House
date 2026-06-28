# My House - MVP

## Project Overview
My House is a real estate platform connecting seekers and owners. This repository contains both the backend (Rust) and frontend (TypeScript/React) code in a monorepo structure. The backend provides a robust REST API with PostgreSQL, and the frontend provides a responsive user interface.

## Local Setup Instructions

### Prerequisites
- Docker & Docker Compose
- Rust (Cargo)
- Node.js (npm or yarn)

### Backend Setup
1. Navigate to the backend directory:
   ```bash
   cd backend
   ```
2. Copy the environment variables:
   ```bash
   cp .env.example .env
   ```
3. Run the development server:
   ```bash
   cargo run
   ```

### Frontend Setup
1. Navigate to the frontend directory:
   ```bash
   cd frontend
   ```
2. Copy the environment variables:
   ```bash
   cp .env.example .env
   ```
3. Install dependencies and start the development server:
   ```bash
   npm install
   npm run dev
   ```

## Branch & Commit Conventions
- **Branching Strategy**: We follow **Trunk-based development**. Use short-lived feature branches and merge into the main branch frequently.
- **Commit Messages**: We strictly use **[Conventional Commits](https://www.conventionalcommits.org/)**. 
  - Examples:
    - `feat: add user login`
    - `fix: resolve crash on startup`
    - `chore: update dependencies`
    - `docs: update readme instructions`
