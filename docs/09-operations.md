Here’s a clear, practical guide for **`09-operations.md`**—everything you and your teammates need for day-to-day operation, management, and troubleshooting of your Docker Compose-based hosting platform.

---

````md
# 09. Operations: Running, Stopping, Updating, and Troubleshooting

## Purpose

This document covers the key commands and workflows for starting, stopping, updating, and monitoring your Docker Compose stack, plus basic troubleshooting tips.

---

## Starting the Full Stack

Start all services (build if needed, run detached):

```bash
docker-compose up -d --build
````

* `-d`: Run in the background (detached)
* `--build`: Rebuild images if source has changed

---

## Stopping All Services

Shut down everything and remove containers:

```bash
docker-compose down
```

* Leaves named volumes/data intact.
* Add `-v` to remove volumes too (be careful!): `docker-compose down -v`

---

## Viewing Logs

View real-time logs from all services:

```bash
docker-compose logs -f
```

* Add a service name for just one service:
  `docker-compose logs -f api`

---

## Checking Status

See which containers are running and their ports:

```bash
docker-compose ps
```

---

## Restarting a Single Service

Rebuild and restart just the web app, for example:

```bash
docker-compose up -d --build web
```

---

## Stopping/Starting Individual Services

Stop just the API:

```bash
docker-compose stop api
```

Start it again:

```bash
docker-compose start api
```

---

## Updating the Stack (Pull + Deploy)

**Common update pattern:**

```bash
git pull
docker-compose up -d --build
```

Or use a simple script, e.g. `deploy.sh`:

```bash
#!/bin/bash
git pull
docker-compose up -d --build
```

---

## Viewing/Managing Volumes

List all volumes:

```bash
docker volume ls
```

Inspect a specific volume:

```bash
docker volume inspect <volume_name>
```

Remove unused volumes:

```bash
docker volume prune
```

---

## Cleaning Up

Remove stopped containers and dangling images:

```bash
docker system prune
```

> Use with caution: this removes all stopped containers, unused networks, dangling images, and build cache.

---

## Troubleshooting

* **App won’t start:**
  Check `docker-compose logs <service>`.
* **Port already in use:**
  Another process is using the port—change Compose config or stop the other process.
* **File changes not reflected:**
  Rebuild the affected service: `docker-compose up -d --build <service>`
* **Database resets:**
  Make sure volumes are configured correctly.
* **Network errors:**
  Ensure services use Docker service names, not `localhost`.

---

## Useful Shortcuts

* **See running containers:**
  `docker ps`
* **See all containers:**
  `docker ps -a`
* **Exec into a running container:**
  `docker exec -it <container_name> /bin/sh` (or `/bin/bash`)

---

## Next Steps

* [x] [10-local-domains-ssl.md](./10-local-domains-ssl.md): (Optional) Custom local domains and SSL for a production-like local environment.
* [x] [11-monitoring-logs.md](./11-monitoring-logs.md): (Optional) Adding logging and basic monitoring.

---

```

---

**How to use:**  
- Paste into `docs/09-operations.md`
- Expand with real-life scenarios, deploy scripts, or CI/CD workflow as your stack grows.

Ready for **step 10 (Local Domains & SSL, optional)?**
```
