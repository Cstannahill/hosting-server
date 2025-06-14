# Fullstack App Hosting Platform

This repository contains a lightweight platform for hosting multiple fullstack applications. Each app defines its own Compose file externally and is reverse proxied through a single NGINX instance. The platform can proxy Python (FastAPI), Node.js (Express or NestJS), and even C# backends.

Sample backends are provided for FastAPI, Express, NestJS and ASP.NET to demonstrate that any Dockerised language can be integrated.

The example apps now include compression middleware and request logging. The React frontend checks the API health endpoint from a configurable URL, and the static file proxy serves assets with long-lived caching headers.

The FastAPI example now includes a flexible `start.sh` script that reads environment variables (`PORT`, `WORKERS`, and `LOG_LEVEL`) and a `/health` endpoint for basic monitoring.

## Usage

1. Add YAML files under `compose/app-registry/` describing each app's domains and ports.
2. Generate the NGINX configuration and reload the proxy:
   ```bash
   python scripts/generate-nginx.py --reload
   ```
3. Start the core services:
   ```bash
   docker-compose up -d --build
   ```
4. Launch each application using its own Compose file or let the helper script manage them for you:
   ```bash
   python scripts/launch_apps.py up
   ```
   The script reads `compose/app-registry/*.yaml` and runs `docker compose` for each app.

The apps will be accessible at the domains specified in the registry.
For the built-in examples:
- React frontend → `http://web.local`
- FastAPI backend → `http://api.local`

The stack now includes a Python-based **data_capture** service. It stores
metrics from `metrics_exporter` in SQLite and embeds them with Ollama's
`nomic-embed-text:v1.5` model, persisting vectors in a local Chroma database.
Embeddings are generated via a dedicated **ollama** container exposed on
`http://ollama:11434`.

See the documentation in the `docs/` directory for detailed guides.

## CLI

A Rust-based CLI tool is available under `cli_tools/custom_cli_tool` to automate common tasks. Build it with `cargo build --release` and run commands like `./target/release/custom_cli_tool deploy`.
