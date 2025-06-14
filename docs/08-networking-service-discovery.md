Excellent! Here’s a thorough, beginner-to-pro-ready template for **`08-networking-service-discovery.md`** covering Docker networking and how your containers/services communicate.

---

````md
# 08. Networking & Service Discovery

## Purpose

This document explains how Docker Compose creates a secure internal network for your app containers and how your services discover and communicate with each other without exposing every port to your host machine or the public internet.

---

## Why Docker Networking?

- **Automatic service discovery:** Containers refer to each other by service name, not IP.
- **Security:** Only exposed ports are accessible from outside; all others are internal.
- **Simplicity:** No need to hardcode or guess IP addresses—Docker handles DNS.
- **Scalability:** Easy to add new services; just reference by name.

---

## How Docker Compose Networking Works

- **Default network:**  
  When you run `docker-compose up`, Docker automatically creates a bridge network for your project (unless you specify otherwise).
- **Service name = hostname:**  
  Each service in your `docker-compose.yml` can be accessed from other services by its name.

### **Example**

```yaml
services:
  api:
    build: ./apps/api
    ...
  db:
    image: postgres:16
    ...
````

* The `api` service can connect to the database using `db:5432` (not `localhost:5432`!).

---

## Example: FastAPI Backend Connecting to Postgres

**In your Python app:**

```python
import os

POSTGRES_HOST = os.environ.get("POSTGRES_HOST", "db")
POSTGRES_PORT = os.environ.get("POSTGRES_PORT", "5432")
```

* Set `POSTGRES_HOST=db` in your `.env` or code.

---

## Example: NGINX Reverse Proxy

* NGINX container proxies to `web:3000` and `api:8000`—those service names resolve to the right containers via Docker’s built-in DNS.

**Sample NGINX config:**

```nginx
proxy_pass http://api:8000/;
proxy_pass http://web:3000/;
```

---

## Ports: Internal vs. External

* **Internal ports:**
  Ports used between containers (e.g., `db:5432`, `api:8000`) are NOT exposed to your host by default.
* **External ports:**
  Only services with a `ports:` section are reachable from your host. For example:

  ```yaml
  ports:
    - "8000:8000"
  ```

  This exposes container port 8000 as host port 8000.

---

## Custom Networks (Optional/Advanced)

* By default, all services in the same Compose file are on the same network.
* You can define multiple networks for more advanced scenarios (multi-project isolation, custom subnetting, etc.).

---

## Useful Commands

* **List all networks:**

  ```bash
  docker network ls
  ```
* **Inspect a network:**

  ```bash
  docker network inspect <network_name>
  ```
* **See running containers and their IPs:**

  ```bash
  docker ps
  docker inspect <container_name>
  ```

---

## Best Practices

* Always connect to other services via their service name, **not** `localhost`.
* Only expose ports that need to be public (e.g., NGINX `80:80`).
* Remove unused ports from Compose for tighter security.
* For multi-project setups, use custom networks to prevent cross-talk.

---

## Troubleshooting

* **App can’t find service:**
  Double-check service names; make sure Compose is running and services are up.
* **Port already in use:**
  Change host port mapping in Compose or free the port on your host.
* **Can’t connect from host:**
  Only services with a `ports:` mapping are exposed to your host.

---

## Further Reading

* [Docker Compose networking docs](https://docs.docker.com/compose/networking/)
* [Docker networking deep dive (Nick Janetakis)](https://nickjanetakis.com/blog/docker-tip-65-understanding-how-docker-container-networks-work-with-examples)

---

## Next Steps

* [ ] [09-operations.md](./09-operations.md): Day-to-day usage, starting/stopping services, viewing logs, and updating containers.

---

```

---

**How to use:**  
- Paste into `docs/08-networking-service-discovery.md`.
- Add custom network diagrams or real-world examples if your stack expands.

Ready for **step 9 (Operations: Start/Stop/Logs/Update)?**
```
