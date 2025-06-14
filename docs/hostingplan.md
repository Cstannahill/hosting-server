# Fullstack App Hosting Platform

This platform allows you to host multiple fullstack applications by referencing their external Compose files.

## Overview

This self-hosted platform is designed to serve as a lightweight PaaS. It allows you to register and host multiple fullstack apps (frontend + backend) by defining their configurations in a centralized registry and using NGINX to dynamically route traffic. Each application lives in its own external directory and is deployed independently.

---

## Goals

- ✅ Host external fullstack applications without embedding them
- ✅ Proxy frontend and backend domains using NGINX
- ✅ Use a dynamic registry-based system to configure hosted apps
- ✅ Keep the platform lightweight, modular, and extensible

---

## Step-by-Step Implementation Plan

### 1. **Project Structure Setup**

Organize your platform repository like so:

```
hosting-server/
├── compose/
│   └── app-registry/        # Registry files for each app
├── nginx/
│   ├── nginx.conf.template  # Templated reverse proxy config
│   └── nginx.conf            # Generated live config
├── scripts/
│   └── generate-nginx.py    # Generator for dynamic NGINX config
├── docker-compose.yml       # Platform services
└── README.md
```

---

### 2. **Register External Apps**

Each app gets a YAML file in `compose/app-registry/` that defines:
  - Name
  - Frontend port & domain
  - Backend port & domain
  - Path to the app’s `docker-compose.yml`

Backends can be written in Python (FastAPI), Node.js (Express or NestJS), or C# using ASP.NET Core. As long as the app exposes a port via Docker Compose, the proxy can route to it.

**Example:** `compose/app-registry/app1.yaml`
```yaml
name: app1
frontend:
  port: 3000
  domain: app1.local
backend:
  port: 8000
  domain: api.app1.local
compose_file: /home/youruser/code/my-apps/app1/docker-compose.yml
```

---

### 3. **Generate NGINX Config**

Use Jinja2 templating and a script to convert the registry into a fully working `nginx.conf`:
```bash
python scripts/generate-nginx.py
```
This populates upstream and server blocks for all defined apps.

---

### 4. **Start the Platform**

Launch the NGINX proxy:
```bash
docker-compose up -d --build
```
This brings up only the platform itself.

---

### 5. **Launch the Apps (Externally)**

Each app lives outside the host repo. You can deploy them individually or run them all via a helper script:
```bash
python scripts/launch_apps.py up
```
The script reads the registry and executes `docker compose` for each referenced `compose_file`. Ensure ports match those defined in the registry.

---

### 6. **Access the Apps**

Once everything is running:
- Open `http://app1.local` → frontend
- Open `http://api.app1.local` → backend

To support custom domains or HTTPS, you'll configure DNS and SSL later.

---

## Future Enhancements

- 🔐 Add SSL support using Let’s Encrypt / Traefik
- 🔄 Enable live NGINX reload after config change
- 🧠 Build a CLI or UI for managing the registry
- 📦 Add isolated DB containers or volumes per app
- 📊 Monitoring + health checks for hosted apps

---

## Requirements

- Docker & Docker Compose
- Python 3 with:
  - `pyyaml`
  - `jinja2`

Install them with:
```bash
pip install pyyaml jinja2
```

---

Let me know if you'd like any part of this scaffolded, converted into a CLI tool, or enhanced with deployment automation.

This setup positions your project as a lightweight but powerful self-hosted PaaS.
