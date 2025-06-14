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
   COPY . .
   RUN pip install fastapi uvicorn
   EXPOSE 8000
   CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]
   ```
2. Define a compose file exposing the port:
   ```yaml
   services:
     api:
       build: .
       ports:
         - "8000:8000"
   ```

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
