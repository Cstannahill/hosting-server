Here’s a thorough, dev-centric doc for **`19-custom-cli-tool.md`** (Rust-based custom CLI tool), styled to match your other service docs:

---

```md
# 19. Rust Project: Custom CLI Tool

## Purpose

This document covers the planning, usage, and integration of a Rust-based CLI tool to help automate and orchestrate DevOps, deployment, and maintenance tasks for your hosting stack.  
The CLI is not a persistent service but a binary you run manually or from scripts/CI.

---

## Why Add a Custom CLI Tool?

- **Productivity:** Rapidly automate complex, repetitive, or multi-step operations (deploy, backup, logs).
- **Consistency:** Ensure the same steps are run every time—by any developer or automation.
- **Extensibility:** Add more commands/features as your platform grows.
- **Speed:** Rust makes it blazing fast and easy to distribute as a static binary.

---

## Key Features

- Subcommands for common tasks:
  - Deploy/redeploy all or individual services
  - Tail or summarize logs
  - Run/schedule backups
  - Check stack status/health
  - Manage environment variables or secrets
  - Custom one-off tasks
- Supports interactive prompts and argument parsing.
- Easy to use in local dev, CI, or remote servers.
- (Optional) Colorful output, confirmation prompts, spinners, etc.

---

## System Architecture

```

+---------------------+          +---------------------+
\|  Developer / CI     | <------> |  Rust CLI Tool      |
\|  (Manual or Script) |          |  ("orchestrator")   |
+---------------------+          +---------------------+
\|    ^                         |
v    |     Runs Docker Compose, hits API, manages .env
+-----------------------------+
\|     Docker Compose Stack     |
+-----------------------------+

```

---

## Directory Structure

```

cli\_tools/
└── custom\_cli\_tool/
├── src/
│   └── main.rs
├── Cargo.toml
└── README.md

````

---

## Implementation Outline

- **Language:** Rust
- **Libraries:**
  - [clap](https://docs.rs/clap/) for argument parsing and subcommands
  - [indicatif](https://docs.rs/indicatif/) for progress bars/spinners
  - [colored](https://docs.rs/colored/) for colorized output
  - [reqwest](https://docs.rs/reqwest/) for HTTP API calls
  - [std::process::Command](https://doc.rust-lang.org/std/process/struct.Command.html) for running Docker Compose, shell, or OS commands

---

## Example Subcommands

- `deploy` — Build and start all services (runs `docker-compose up -d --build`)
- `logs [SERVICE]` — Tail logs from one or all containers (wraps `docker-compose logs -f`)
- `backup` — Triggers backup via API or by calling the backup script
- `status` — Calls healthcheck API or summarizes running containers
- `env [get/set]` — Manipulates `.env` file safely

---

## Usage

1. **Build the CLI tool:**
   ```bash
   cd cli_tools/custom_cli_tool
   cargo build --release
````

2. **Run directly:**

   ```bash
   ./target/release/custom_cli_tool deploy
   ./target/release/custom_cli_tool logs api
   ./target/release/custom_cli_tool status
   ./target/release/custom_cli_tool env get POSTGRES_DB
   ```
3. **Install to system path (optional):**

   ```bash
   cp ./target/release/custom_cli_tool /usr/local/bin/mycli
   ```
4. **Use in scripts/CI:**

   * Add as a build step or hook in your pipeline.

---

## Example CLI Output

```
$ mycli deploy
[INFO] Building Docker images...
[✓] Images built.
[INFO] Starting all services...
[✓] All services started. Stack is healthy.

$ mycli logs api
[api] 2024-06-12T19:31:33Z [INFO] Started FastAPI on 0.0.0.0:8000

$ mycli status
Service        Status    Ports
---------      ------    ------
web            running   3000->3000
api            running   8000->8000
db             running   5432->5432
```

---

## Example Compose Integration

* The CLI does **not** need to be defined as a service in Compose.
* It interacts with the stack via:

  * `docker-compose` (shell commands)
  * `.env` file (direct read/write)
  * HTTP APIs (`healthcheck_dashboard`, etc.)

---

## Optional Extensions

* Interactive menu for selecting actions (`dialoguer` crate)
* Support for plugins (user-defined commands/scripts)
* Secrets management (encrypt/decrypt .env values)
* Self-update mechanism (download new binary)

---

## Troubleshooting

* **Permission errors:** Ensure user running CLI can access Docker socket and files.
* **Subcommand errors:** Run with `--help` for usage details.
* **Not found:** Add binary to your `$PATH`.

---

## Further Reading

* [clap CLI parser](https://docs.rs/clap/)
* [indicatif progress indicators](https://docs.rs/indicatif/)
* [Rust scripting and shell integration](https://doc.rust-lang.org/std/process/)

---

## Next Steps

* Add more subcommands as your needs grow.
* Document all custom tasks and flags in this file or a generated `--help` output.

---

**Congrats! This completes your Rust microservices and tools documentation set.**

```

---

Let me know if you want a Table of Contents doc, a summary, code/project scaffolds for any service, or to go deeper into Rust CLI tool best practices!
```
