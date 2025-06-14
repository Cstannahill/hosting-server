# docker-compose.yml

version: "3.8"

services:
  nginx:
    image: nginx:latest
    container_name: nginx
    ports:
      - "80:80"
    volumes:
      - ./nginx/nginx.conf:/etc/nginx/nginx.conf:ro
    depends_on: []

# compose/app-registry/app1.yaml

name: app1
frontend:
  port: 3000
  domain: app1.local
backend:
  port: 8000
  domain: api.app1.local
compose_file: /home/youruser/code/my-apps/app1/docker-compose.yml

# compose/app-registry/web.yaml

name: web
frontend:
  port: 3000
  domain: web.local
compose_file: ../apps/web/docker-compose.yml

# compose/app-registry/api.yaml

name: api
backend:
  port: 8000
  domain: api.local
compose_file: ../apps/api/docker-compose.yml

# nginx/nginx.conf.template

worker_processes 1;
events { worker_connections 1024; }

http {
  {{#each apps}}
  upstream {{this.name}}_frontend {
    server {{this.name}}_frontend:{{this.frontend.port}};
  }

  upstream {{this.name}}_backend {
    server {{this.name}}_backend:{{this.backend.port}};
  }

  server {
    listen 80;
    server_name {{this.frontend.domain}};

    location / {
      proxy_pass http://{{this.name}}_frontend;
      proxy_set_header Host $host;
      proxy_set_header X-Real-IP $remote_addr;
    }
  }

  server {
    listen 80;
    server_name {{this.backend.domain}};

    location / {
      proxy_pass http://{{this.name}}_backend;
      proxy_set_header Host $host;
      proxy_set_header X-Real-IP $remote_addr;
    }
  }
  {{/each}}
}

# scripts/generate-nginx.py

import yaml
import os
from jinja2 import Template

REGISTRY_PATH = "compose/app-registry"
TEMPLATE_PATH = "nginx/nginx.conf.template"
OUTPUT_PATH = "nginx/nginx.conf"

def load_registry():
    apps = []
    for file in os.listdir(REGISTRY_PATH):
        if file.endswith(".yaml"):
            with open(os.path.join(REGISTRY_PATH, file)) as f:
                apps.append(yaml.safe_load(f))
    return apps

def main():
    apps = load_registry()

    with open(TEMPLATE_PATH) as f:
        template = Template(f.read())

    rendered = template.render(apps=apps)

    with open(OUTPUT_PATH, "w") as f:
        f.write(rendered)

    print(f"✅ nginx.conf updated with {len(apps)} app(s)")

if __name__ == "__main__":
    main()

# README.md

# Fullstack App Hosting Platform

This platform allows you to host multiple fullstack applications by referencing their external Compose files.

## Usage

1. Create `compose/app-registry/*.yaml` entries for each app.
2. Run the generator (optionally reload NGINX automatically):
   ```bash
   python scripts/generate-nginx.py --reload
   ```
3. Start the core platform (NGINX):
   ```bash
   docker-compose up -d --build
   ```
4. Start your apps using their own Compose files or run them all via the helper script:
   ```bash
   python scripts/launch_apps.py up
   ```

Apps will be reverse proxied by domain via NGINX. The platform targets Python (FastAPI), Node.js (Express/NestJS), and C# backends as first-class citizens.
For the provided React and FastAPI samples, browse to `http://web.local` and `http://api.local` once everything is running.

The sample apps demonstrate hosting FastAPI (Python), Express and NestJS (Node.js),
and an ASP.NET minimal API. Any Dockerised backend can be added to the registry
following the same pattern.

The platform also ships with a `data_capture` service which stores metrics in
SQLite, embeds them with the `nomic-embed-text:v1.5` model via Ollama and writes
the vectors to a Chroma database.

## Requirements
- Docker & Docker Compose
- Python (`pyyaml`, `jinja2`)
- Node.js and the .NET SDK if you wish to build the example Node and C# apps locally

---

Let me know if you'd like SSL support or dynamic config reloads next.
