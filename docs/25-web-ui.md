# 25. Web Deployment Interface

## Purpose

This guide introduces a minimal web interface that allows users to deploy and stop applications from the hosting platform. The design is inspired by Vercel's dashboard, providing a simple list of apps and one-click actions to launch them via the new `deployment_api` service.

---

## Features

- Display all registered apps from `compose/app-registry`.
- Deploy or stop an app with a single button.
- Show basic status messages returned from the API.

---

## System Architecture

```
+--------------+      HTTP       +-----------------+    Docker Compose
|  Platform UI |  <--------->   |  deployment_api  |  <---------------> host
+--------------+                 +-----------------+
```

1. The React based **Platform UI** calls the FastAPI **deployment_api**.
2. The API reads the registry and runs `docker compose` commands on the host.
3. NGINX continues to proxy traffic for running apps as before.

---

## Directory Structure

```
apps/
  platform-ui/       # React frontend
services/
  deployment_api/    # FastAPI backend to run compose commands
```

---

## Platform UI Usage

1. Install dependencies and start in development mode:
   ```bash
   cd apps/platform-ui
   npm install
   npm run dev
   ```
   The UI runs on `http://localhost:3001` and proxies API calls to `deployment_api`.

2. Build and run with Docker Compose:
   ```bash
   docker compose up -d deployment_api
   npm run build
   ```
   After building, serve the static files with any web server or integrate it into your existing NGINX setup.

---

## deployment_api Service

- **Endpoint:** `GET /apps` – list available app names.
- **Endpoint:** `POST /deploy/{name}` – build and start an app.
- **Endpoint:** `POST /stop/{name}` – stop an app and remove its containers.
- The service mounts the repository directory and Docker socket so it can execute `docker compose` commands.

---

## Next Steps

- Add authentication and per-user access controls.
- Display real-time deployment logs in the UI.
- Track deployment history for each app.

