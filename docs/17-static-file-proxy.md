Here’s a complete, production-style doc for **`17-static-file-proxy.md`** (Rust-based static file server and/or programmable proxy), matching your documentation style:

---

```md
# 17. Rust Service: Static File Server / Proxy

## Purpose

This document outlines the design and usage of a Rust-based static file server and programmable reverse proxy for your hosting stack.  
This service can efficiently serve static files (docs, assets, downloads) and/or forward requests to other services, optionally with filtering or routing rules.

---

## Why Add a Static File Server or Proxy?

- **Performance:** Serve static files faster than from your main app containers.
- **Flexibility:** Centralize asset hosting, downloads, or even static documentation.
- **Programmability:** Add request/response filters, simple authentication, or path rewriting.
- **Separation of concerns:** Keep static serving out of your main API/web app logic.

---

## Key Features

- Serve files from a specified directory via HTTP.
- Configurable root path, allowed file types, and cache headers.
- (Optional) Programmable reverse proxy: forward requests to other backend services.
- (Optional) Middleware: basic auth, CORS, path rewrite, etc.
- (Optional) Hot-reload for new/updated files.

---

## System Architecture

```

```
      [Browser]
          |
     +----v-----+
     |  Static  |      (or)   +------------+
     | File     |------------>|   Proxy    |--> [API / External]
     | Server   |   (option)  | (Routing)  |
     +----------+             +------------+
```

```
- You can use this as a drop-in for serving `/public`, `/docs`, or as a simple file download host.
- The same binary can optionally run in "proxy mode" (forward API requests with custom logic).

---

## Directory Structure

```

services/
└── static\_file\_proxy/
├── src/
│   └── main.rs
├── Cargo.toml
├── Dockerfile
└── config.toml (optional)

````

---

## Implementation Outline

- **Language:** Rust (with async IO for high performance)
- **Libraries:**
  - [axum](https://github.com/tokio-rs/axum) or [warp](https://github.com/seanmonstar/warp) for HTTP
  - [tokio](https://tokio.rs/) for async runtime
  - [serde](https://serde.rs/) for config
  - [hyper-reverse-proxy](https://github.com/sozu-proxy/hyper-reverse-proxy) or custom for proxy features

---

## Dockerfile Example

```dockerfile
# services/static_file_proxy/Dockerfile

FROM rust:1.76 as builder

WORKDIR /usr/src/static_file_proxy
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /usr/src/static_file_proxy/target/release/static_file_proxy /usr/local/bin/static_file_proxy
COPY config.toml /app/config.toml
CMD ["static_file_proxy"]
````

---

## Docker Compose Integration

Add to `docker-compose.yml`:

```yaml
static_file_proxy:
  build: ./services/static_file_proxy
  container_name: static_file_proxy
  ports:
    - "9200:8080"
  volumes:
    - ./public:/app/public:ro           # Host directory for static files
  environment:
    - STATIC_ROOT=/app/public
    - PROXY_MODE=off                    # "on" to enable reverse proxy mode
    - PROXY_TARGET=http://api:8000      # Optional: backend to forward API calls
    - CONFIG_PATH=/app/config.toml
```

---

## Configuration

* **Via env vars or config file.**
* Example `config.toml`:

  ```toml
  [server]
  port = 8080
  static_root = "/app/public"
  proxy_mode = false
  allowed_extensions = ["html", "css", "js", "png", "jpg", "pdf"]
  cache_seconds = 600

  [proxy]
  enabled = true
  base_path = "/api/"
  target = "http://api:8000"
  path_rewrite = true
  add_auth_header = false
  ```

---

## Usage

1. **Build locally for dev:**

   ```bash
   cd services/static_file_proxy
   cargo run
   ```
2. **Build container:**

   ```bash
   docker build -t static-file-proxy .
   ```
3. **Run in Compose:**

   ```bash
   docker-compose up -d static_file_proxy
   ```
4. **Visit:**

   * [http://localhost:9200/](http://localhost:9200/) (static files)
   * [http://localhost:9200/api/some-endpoint](http://localhost:9200/api/some-endpoint) (proxied if enabled)

---

## Example Features

* **Static:**

  * Serves `/public/docs/readme.pdf` at `http://localhost:9200/docs/readme.pdf`
  * Sets cache headers
* **Proxy:**

  * Requests to `/api/*` forwarded to `api:8000`
  * Supports path rewriting if needed

---

## Optional Extensions

* Basic/bearer auth or API key support
* CORS configuration per route
* Gzip or Brotli compression
* Logging and access logs

---

## Troubleshooting

* **404 errors:** Check volume mounts and static root path.
* **Permission errors:** Ensure `public` dir has correct read perms.
* **Proxy not working:** Confirm backend URL, Docker networking, and proxy config.

---

## Further Reading

* [axum static file serving](https://docs.rs/axum/latest/axum/struct.Router.html#method.route_service)
* [warp static file serving](https://docs.rs/warp/latest/warp/filters/fs/index.html)
* [hyper-reverse-proxy](https://github.com/sozu-proxy/hyper-reverse-proxy)

---

## Next Service

[18-metrics-exporter.md](./18-metrics-exporter.md): Rust-based custom metrics exporter (Prometheus format).

---

```

---

**Ready for the next doc (18-metrics-exporter), or want to adjust this one?**
```
