Great! Here’s a **detailed, production-ready template for `02-dockerizing-web.md`**—your Vite/React frontend.

---

````md
# 02. Dockerizing the Frontend (Vite/React)

## Purpose

This document explains how to containerize the frontend app using Docker, why containerization is valuable, and includes a step-by-step guide with best practices for building, running, and testing your Vite (React/TypeScript) app in a consistent, portable way.

---

## Why Dockerize the Frontend?

- **Consistency:** Same environment everywhere (dev, staging, prod)
- **Isolation:** Avoid “works on my machine” issues
- **Portability:** Deploy easily to local server, VPS, or the cloud
- **Easy orchestration:** Integrate smoothly with other services in Docker Compose

---

## Dockerfile Explained

**Location:**  
`apps/web/Dockerfile`

### **Recommended: Multi-Stage Build for Static Hosting**

1. **Stage 1:** Build the production static files using Node.js
2. **Stage 2:** Serve them efficiently with NGINX

```dockerfile
# Stage 1: Build frontend assets
FROM node:20-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .
RUN npm run build

# Stage 2: Serve with NGINX
FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
EXPOSE 3000
````

* **Why this pattern?**

  * Keeps the final image very small and secure (no Node, just static files and NGINX)
  * Fast builds and reloads in dev and prod

---

## Step-by-Step: Build and Run Locally

1. **Navigate to the frontend directory:**

   ```bash
   cd apps/web
   ```

2. **Build the Docker image:**

   ```bash
   docker build -t myapp-web .
   ```

   *(Replace `myapp-web` with any name you want for the image.)*

3. **Run the container:**

   ```bash
   docker run --rm -it -p 3000:3000 myapp-web
   ```

   * Visit [http://localhost:3000](http://localhost:3000) to see your app!

---

## Integrating with Docker Compose

When using Docker Compose, the service will be defined something like this:

```yaml
services:
  web:
    build: ./apps/web
    container_name: web
    ports:
      - "3000:3000"
    environment:
      - NODE_ENV=production
```

> **Note:**
> The NGINX container listens on port 3000 by default (as set in its config). If you want to serve on a different port, adjust the `EXPOSE` line and your Compose config.

---

## Development Tips

* **For local dev:** You usually run Vite’s dev server *outside* of Docker (`npm run dev`).
  Docker is mainly for running the production build.
* **Hot reload:** If you want hot reload inside Docker, you can mount your source code as a volume and run `npm run dev` in the container, but this is optional and advanced.
* **Static assets only:** This pattern is for SPAs or static exports. If you use SSR, use a Node-based server in Stage 2 instead of NGINX.

---

## Troubleshooting

* **Build errors:** Double-check `package.json` and `vite.config.js` for correct build/output settings.
* **Blank page:** Make sure `nginx.conf` is not trying to serve `/api` or other routes—leave that for the main reverse proxy.
* **Port conflicts:** Ensure only one service is trying to use port 3000 on your host at a time.

---

## Next Steps

* [ ] [03-dockerizing-api.md](./03-dockerizing-api.md): Dockerizing the backend (FastAPI) app.

```

---

**How to use:**  
- Copy/paste into `docs/02-dockerizing-web.md`.
- Update `myapp-web` image name as you like.
- Adjust “Development Tips” section if you have a custom workflow.

**Ready for step 3 (Dockerizing API), or want a tweak to this one?**
```
