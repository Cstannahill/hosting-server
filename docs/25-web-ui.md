# 25. Web Deployment Interface

## Purpose

This guide introduces a minimal web interface that allows users to deploy and stop applications from the hosting platform. The design is inspired by Vercel's dashboard, providing a simple list of apps and one-click actions to launch them via the new `deployment_api` service.

---

## Features

- Display all registered apps from `compose/app-registry`.
- Deploy or stop an app with a single button.
- Show basic status messages returned from the API.

### Design Goals

The UI should remain lightweight while borrowing the clean aesthetics of Vercel's dashboard. Actions must be discoverable with minimal clicks and every operation should provide instant feedback. The layout uses a simple dark theme and responsive components so the interface works on desktops and mobile devices.

### Aesthetics & Layout

- **Dark mode first** with a subtle accent color for buttons and status badges.
- Top navigation bar with platform logo, project switcher and user menu.
- Responsive grid that lists all apps with quick-action buttons.
- Styles implemented with Tailwind or CSS modules to keep the markup lean.

### Pages and Flow

1. **Login** – optional OAuth or basic auth that redirects to the dashboard.
2. **Dashboard** – lists registered apps from the registry with deploy and stop buttons.
3. **App Detail** – shows deployment history, environment variables editor and real-time logs via WebSockets.
4. **Settings** – manage API tokens, SSH keys and domain aliases.

### Developer Experience

- One-click deploy/stop with toast notifications.
- Environment variables stored securely and injected during deployment.
- Live log stream from the API to watch build output.
- API tokens so CI pipelines can trigger deployments.
- Clear error pages and helpful tooltips throughout the UI.

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

