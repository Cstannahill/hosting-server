# Hosting Platform

This repository contains a modular setup for hosting a React/Vite frontend and a FastAPI backend using Docker Compose.

## Quick Start

1. Ensure Docker and Docker Compose are installed.
2. Copy `.env.example` to `.env` and adjust environment variables as needed.
3. Run `docker-compose up --build` to start all services.
4. Visit `http://localhost` in your browser.

See the documentation in the `docs/` directory for detailed guides.

## CLI

A Rust-based CLI tool is available under `cli_tools/custom_cli_tool` to automate common tasks. Build it with `cargo build --release` and run commands like `./target/release/custom_cli_tool deploy`.
