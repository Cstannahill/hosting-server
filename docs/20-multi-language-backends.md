# 20. Hosting Multiple Language Backends

## Purpose

This guide explains how the platform can host APIs written in different languages. The
sample apps in `apps/` demonstrate Python (FastAPI), Node.js (Express and NestJS), and
C# (.NET) services. Each backend is containerised with its own `Dockerfile` and optional
`docker-compose.yml` for local testing.

## Directory Layout

```
apps/
  api/           # FastAPI example
  express/       # Node.js Express
  nest/          # Node.js NestJS
  csharp-api/    # ASP.NET minimal API
```

## General Steps

1. Implement your backend using your language/framework of choice.
2. Provide a `Dockerfile` that exposes the service on a configurable `PORT`.
3. (Optional) Include a `docker-compose.yml` for local development.
4. Add an entry under `compose/app-registry/` pointing to the compose file and desired domains.
5. Regenerate `nginx.conf` and reload the proxy:
   ```bash
   python scripts/generate-nginx.py --reload
   ```

The registry-driven approach means any language can be supported as long as it is Dockerised.

## Example Compose Entry

```yaml
name: express
frontend:
  port: 4000
  domain: express.local
backend:
  port: 4000
  domain: api.express.local
compose_file: ../apps/express/docker-compose.yml
```

This configuration forwards `express.local` to the frontend container and `api.express.local`
to the backend service once the compose file is launched separately.

---

With this scaffolding in place you can expand to additional languages or frameworks as needed.
