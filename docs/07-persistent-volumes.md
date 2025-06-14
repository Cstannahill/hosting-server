Here’s a thorough, practical template for **`07-persistent-volumes.md`**—covering persistent data, why it matters, and how to use Docker volumes in your stack.

---

````md
# 07. Persistent Data & Docker Volumes

## Purpose

This document explains how to keep your app’s important data (e.g., databases, uploaded files) safe and persistent across container restarts, updates, or host reboots, using Docker volumes.

---

## Why Use Docker Volumes?

- **Persistence:** Containers are ephemeral by design. Volumes ensure data outlives the container lifecycle.
- **Safety:** Prevents data loss on upgrades, rebuilds, or crashes.
- **Portability:** Volumes can be backed up, restored, or migrated between hosts.
- **Separation:** Keeps code, config, and persistent data logically separated.

---

## Types of Docker Volumes

- **Named Volumes:** Managed by Docker; easy, safe, portable.  
  Example: `db_data`
- **Bind Mounts:** Maps a specific host directory into a container.  
  Example: `./mydata:/data`

**Best Practice:** Use **named volumes** for databases and most persistent needs.

---

## How to Define and Use Volumes in Docker Compose

### **1. Defining a Named Volume**

In `docker-compose.yml`:

```yaml
services:
  db:
    image: postgres:16
    volumes:
      - db_data:/var/lib/postgresql/data

volumes:
  db_data:
````

* `db_data` is the volume name.
* `/var/lib/postgresql/data` is the internal path where Postgres stores its data.

### **2. Bind Mount Example (for local file dev only)**

```yaml
services:
  api:
    volumes:
      - ./apps/api/uploads:/app/uploads
```

* Mounts the host’s `apps/api/uploads` folder into the container.

---

## How Docker Volumes Work

* **Data persists even if you stop/remove the container.**
* Data is lost only if you explicitly remove the volume (`docker volume rm ...`).

**Tip:** You can inspect existing volumes:

```bash
docker volume ls
docker volume inspect db_data
```

---

## Backing Up and Restoring Volumes

**Backup:**

```bash
docker run --rm -v db_data:/volume -v $(pwd):/backup alpine tar czf /backup/pgdata.tar.gz -C /volume . 
```

**Restore:**

```bash
docker run --rm -v db_data:/volume -v $(pwd):/backup alpine tar xzf /backup/pgdata.tar.gz -C /volume
```

* Replace `db_data` and backup path as needed.

---

## When to Use Bind Mounts

* **Development:** For hot-reloading code or sharing uploads.
* **Not for production:** Host directory permissions, paths, and structure can cause surprises.

---

## Cleaning Up Volumes (Caution!)

To remove **unused** volumes (not referenced by any container):

```bash
docker volume prune
```

**To remove a specific volume:**

```bash
docker volume rm db_data
```

> **Warning:** Removing a volume **deletes all data** stored in it.
> Double-check before running these commands.

---

## Troubleshooting

* **Database resets after container restart:**
  Check your volume definitions and paths.
* **Permissions errors:**
  The Docker daemon runs as root; if you use bind mounts, ensure host directories have proper permissions.

---

## Best Practices

* Use named volumes for all databases and anything you can’t afford to lose.
* Back up critical volumes regularly.
* Never use ephemeral (`tmpfs`) or unmounted directories for production data.

---

## Next Steps

* [ ] [08-networking-service-discovery.md](./08-networking-service-discovery.md): How your services find and talk to each other on the Docker network.

---

```

---

**How to use:**  
- Copy this into `docs/07-persistent-volumes.md`.
- Customize volume names, internal paths, and backup routines to match your stack as needed.

Ready for **step 8 (Networking & Service Discovery)?**
```
