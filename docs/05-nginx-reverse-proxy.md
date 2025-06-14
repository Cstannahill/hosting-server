Awesome! Here’s a detailed template for **`05-nginx-reverse-proxy.md`**, covering the why, how, and best practices for your NGINX reverse proxy setup.

---

````md
# 05. NGINX Reverse Proxy Setup

## Purpose

This document explains how to set up and configure NGINX as a reverse proxy for your local hosting platform.  
A reverse proxy routes incoming requests from a single public entry point (e.g., `localhost:80`) to the correct backend service (frontend, API, etc.), enabling clean URLs, service isolation, and flexible routing.

---

## Why Use a Reverse Proxy?

- **Single entry point:** All apps accessible from one port/domain.
- **Clean URLs:** No need to remember ports (e.g., `/` for frontend, `/api` for backend).
- **Service isolation:** Frontend and backend run on internal ports, not directly exposed.
- **Routing flexibility:** Easy to add more apps or change paths.
- **SSL Termination:** (Advanced) Central place to manage HTTPS.

---

## Where to Find the Config

**Location in project:**  
`nginx/nginx.conf`

You can mount this file into the NGINX container using Docker Compose.

---

## Example NGINX Config

This example assumes:
- Frontend (web) is available at `web:3000` (internal Docker service name and port)
- Backend (api) is available at `api:8000`
- NGINX listens on port 80

```nginx
events {}

http {
    server {
        listen 80;

        # Frontend: all other routes go here
        location / {
            proxy_pass http://web:3000;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
        }

        # Backend: all /api/ routes go here
        location /api/ {
            proxy_pass http://api:8000/;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
        }
    }
}
````

* The `/api/` path is **proxied** to your FastAPI backend.
* All other paths go to your frontend (React/Vite app).

---

## Integrating with Docker Compose

**Sample Compose service (from `docker-compose.yml`):**

```yaml
  nginx:
    image: nginx:latest
    container_name: nginx
    ports:
      - "80:80"
    volumes:
      - ./nginx/nginx.conf:/etc/nginx/nginx.conf:ro
    depends_on:
      - web
      - api
```

* The `volumes` line mounts your custom config inside the container.
* Only NGINX exposes a port to the host (port 80).

---

## How Requests Flow

```
[Browser: http://localhost] 
        │
        ▼
[NGINX:80]
 ├─► /api/  →  [api:8000]
 └─► /      →  [web:3000]
```

---

## How to Add More Apps or Routes

* **Add a new service in Compose** (e.g., `blog`)
* **Add a new location block** in `nginx.conf`:

  ```nginx
  location /blog/ {
      proxy_pass http://blog:4000/;
      proxy_set_header Host $host;
      proxy_set_header X-Real-IP $remote_addr;
  }
  ```
* **(Optional)** Set up subdomains with `server_name` for advanced routing.

---

## Tips & Gotchas

* **Order matters:** Place more specific `location` blocks (like `/api/`) above catch-alls (`/`).
* **Trailing slashes:** Ensure `proxy_pass` has a `/` at the end for path consistency.
* **Headers:** Always forward `Host` and `X-Real-IP` for proper logging and CORS.
* **Hot-reloading config:** You can reload NGINX in the container without stopping all services:

  ```bash
  docker exec nginx nginx -s reload
  ```

---

## Troubleshooting

* **404s:** Double-check that the `proxy_pass` targets the correct internal service name and port.
* **CORS errors:** Make sure backend sets appropriate CORS headers if accessed from frontend.
* **Port conflicts:** Only NGINX should bind to port 80; all other ports should be internal.

---

## Next Steps

* [ ] [06-env-vars-secrets.md](./06-env-vars-secrets.md): Managing environment variables and secrets.
* [ ] [07-persistent-volumes.md](./07-persistent-volumes.md): Ensuring data persists with volumes.

---

```

---

**How to use:**  
- Copy to `docs/05-nginx-reverse-proxy.md`
- Adjust internal service names/ports or add extra location blocks as needed.
- Add any project-specific tips or custom routes as you expand.

Ready for **step 6 (Environment Variables & Secrets)**?
```
