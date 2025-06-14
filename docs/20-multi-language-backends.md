# 20. Hosting Different Backend Languages

## Purpose

This guide demonstrates how to containerize and run various backend technologies with the hosting platform. Any language that can be packaged in a Docker image can be proxied by NGINX.

The examples below show minimal setups for Python (FastAPI), Node.js (Express and NestJS), and C# (ASP.NET Core).

---

## Python FastAPI

1. Create your application and add a `Dockerfile` similar to:
   ```Dockerfile
   FROM python:3.12-slim
   WORKDIR /app
   COPY requirements.txt ./
   RUN pip install --no-cache-dir -r requirements.txt
   COPY . .
   RUN chmod +x start.sh prestart.sh
   ENV PORT=8000
   EXPOSE $PORT
   CMD ["./start.sh"]
   ```
2. Define a compose file exposing the port:
   ```yaml
   services:
     api:
       build: .
       ports:
         - "8000:8000"
       environment:
         - PORT=8000
   ```

`start.sh` reads the `PORT`, `WORKERS`, and `LOG_LEVEL` variables to control the Uvicorn server.

---

## Node.js Express

1. Basic server (`index.js`):
   ```javascript
   const express = require('express');
   const app = express();
   app.get('/', (req, res) => res.json({ message: 'Hello from Express' }));
   app.listen(4000);
   ```
2. `Dockerfile`:
   ```Dockerfile
   FROM node:20-alpine
   WORKDIR /app
   COPY package.json ./
   RUN npm install --production
   COPY . .
   EXPOSE 4000
   CMD ["node", "index.js"]
   ```

---

## Node.js NestJS

1. Minimal `main.ts` using NestJS decorators.
2. Build with TypeScript and run the compiled output.
3. Compose file exposes port `4001` just like the Express example.

---

## C# ASP.NET Core

1. Create a simple `Program.cs`:
   ```csharp
   var builder = WebApplication.CreateBuilder(args);
   var app = builder.Build();
   app.MapGet("/", () => Results.Json(new { message = "Hello from ASP.NET Core" }));
   app.Run();
   ```
2. `Dockerfile` targeting .NET 8:
   ```Dockerfile
   FROM mcr.microsoft.com/dotnet/aspnet:8.0 AS base
   WORKDIR /app
   EXPOSE 5000

   FROM mcr.microsoft.com/dotnet/sdk:8.0 AS build
   WORKDIR /src
   COPY *.csproj ./
   RUN dotnet restore
   COPY . .
   RUN dotnet publish -c Release -o /app/publish

   FROM base AS final
   WORKDIR /app
   COPY --from=build /app/publish .
   ENTRYPOINT ["dotnet", "csharp_api.dll"]
   ```
3. Compose file exposes `5000` for the proxy.

---

Once Docker images for these services are built and compose files are referenced in `compose/app-registry/*.yaml`, run:
```bash
python scripts/generate-nginx.py
python scripts/launch_apps.py up
```
NGINX will proxy to each backend according to the registry configuration.

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
