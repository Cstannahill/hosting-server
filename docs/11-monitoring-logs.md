Here’s a clear and actionable template for **`11-monitoring-logs.md`**—covering both basic log handling (for every stack) and optional, scalable monitoring solutions as your needs grow.

---

````md
# 11. Monitoring & Logs

## Purpose

This document explains how to view and manage logs from all your services, and introduces simple ways to add basic monitoring and advanced observability to your Docker Compose stack.

---

## 1. Service Logs (The Basics)

Docker Compose lets you access logs from every container.

### **A. View Logs for All Services**

```bash
docker-compose logs -f
````

* `-f` (follow): Stream logs live, like `tail -f`.

### **B. View Logs for a Single Service**

```bash
docker-compose logs -f api
docker-compose logs -f web
```

### **C. Access Logs in Running Containers**

```bash
docker exec -it <container_name> sh
# Then use 'cat' or 'less' to view files in /var/log/ or other app locations.
```

---

## 2. Where Logs Go (by Default)

* **API logs:**
  Output by FastAPI/Uvicorn to stdout (captured by Docker).
* **Web logs:**
  NGINX logs requests to `/var/log/nginx/` in the container, and errors are sent to stdout/stderr.
* **DB logs:**
  Postgres, Mongo, etc., write to stdout (Docker will capture them).

---

## 3. Log Rotation and Retention

Docker itself does basic log rotation by default. For heavy production loads, configure [log drivers](https://docs.docker.com/config/containers/logging/configure/).

---

## 4. Advanced: Centralized Log Aggregation

As your platform grows, it’s helpful to have a central place to search and visualize logs across all services.

### **A. Loki (by Grafana Labs) + Grafana**

* [Grafana Loki](https://grafana.com/oss/loki/) is a lightweight, open-source log aggregation system.
* **How to use:**

  * Add Loki, Promtail (log shipper), and Grafana services to `docker-compose.yml`.
  * Configure Promtail to scrape logs from your containers.
  * Use Grafana web UI for powerful querying, dashboards, and alerts.

**Example compose services** (minimal!):

```yaml
  loki:
    image: grafana/loki:2.9.0
    ports: ["3100:3100"]

  promtail:
    image: grafana/promtail:2.9.0
    volumes:
      - /var/lib/docker/containers:/var/lib/docker/containers:ro
      - /var/log:/var/log:ro
    command: -config.file=/etc/promtail/config.yml

  grafana:
    image: grafana/grafana:10.4.2
    ports: ["3001:3000"]
```

* **See [Grafana’s official Docker Compose Loki stack](https://github.com/grafana/loki/blob/main/production/docker-compose.yaml)** for full setup and docs.

---

### **B. Alternative Monitoring Stacks**

* **Prometheus:** For metrics (CPU, RAM, request rates, etc.)
* **cAdvisor:** For per-container resource monitoring
* **ELK Stack:** (Elasticsearch, Logstash, Kibana) for large-scale logging (heavyweight, enterprise-scale)

---

## 5. Simple Monitoring

For solo/dev setups, sometimes all you need is:

* **Regularly running `docker stats`** to see resource use:

  ```bash
  docker stats
  ```
* **Health endpoints:**
  Add `/health` endpoints to your API/web services and use scripts or UptimeRobot to check them.
* **Alerts:**
  Set up [Grafana Cloud Free](https://grafana.com/products/cloud/) for quick hosted dashboards.

---

## 6. Best Practices

* Stream logs to stdout/stderr in your app configs—this is what Docker captures.
* Use consistent log formats (JSON, line-based, etc.) for easier searching.
* Secure your monitoring dashboards (Grafana, Kibana) with strong passwords and restrict access to your local network.

---

## Further Reading

* [Docker logging drivers](https://docs.docker.com/config/containers/logging/configure/)
* [Grafana Loki Getting Started](https://grafana.com/docs/loki/latest/get-started/)
* [Prometheus and Docker Compose](https://prometheus.io/docs/prometheus/latest/getting_started/)
* [cAdvisor](https://github.com/google/cadvisor)

---

## Next Steps

* [ ] [12-scaling-migration.md](./12-scaling-migration.md): Planning for scaling and migrating to production or cloud.

---

```

---

**How to use:**  
- Paste into `docs/11-monitoring-logs.md`.
- Add screenshots, specific service log locations, or full monitoring stack configs as your system grows.

Ready for **step 12 (Scaling & Migration)?**
```
