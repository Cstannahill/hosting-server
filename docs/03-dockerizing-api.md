# 03. Dockerizing the Backend (FastAPI/Python)

## Purpose

This guide shows how to create a Docker image for the FastAPI backend so it can be run consistently in any environment and orchestrated with the rest of the stack.

## Why Dockerize the Backend?

- **Consistency:** Same Python environment and dependencies everywhere.
- **Isolation:** Avoid conflicts with system Python packages.
- **Orchestration:** Works seamlessly with other services in Docker Compose.
- **Portability:** Deployable on local machines, VPS, or cloud hosts.

## Dockerfile Explained

**Location:** `apps/api/Dockerfile`

```dockerfile
FROM python:3.12-slim
WORKDIR /app
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt
COPY . .
EXPOSE 8000
CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]
```

- `main:app` assumes the entry point is `main.py` with `app = FastAPI()`.
- `requirements.txt` contains all Python dependencies.

## Step-by-Step: Build and Run Locally

1. Navigate to the backend directory:

   ```bash
   cd apps/api
   ```

2. Build the Docker image:

   ```bash
   docker build -t myapp-api .
   ```

3. Run the container:

   ```bash
   docker run --rm -it -p 8000:8000 myapp-api
   ```

   Visit [http://localhost:8000/docs](http://localhost:8000/docs) to view the interactive API docs.

## Integrating with Docker Compose

In `docker-compose.yml` the service is defined as:

```yaml
services:
  api:
    build: ./apps/api
    container_name: api
    ports:
      - "8000:8000"
    environment:
      - ENV=production
```

Compose will build and run the container automatically.

## Development Tips

- Use `uvicorn main:app --reload --host 0.0.0.0 --port 8000` for hot reloading in development.
- Mount your source directory as a volume if you want live code updates.
- Load sensitive credentials from environment variables rather than hard coding them.

## Troubleshooting

- **Dependency errors:** verify everything is listed in `requirements.txt`.
- **Port issues:** only one service can bind to port `8000` on the host.
- **File changes not reflected:** the production image does not auto-reload; use reload mode during development.

## Next Steps

- [x] [04-docker-compose-orchestration.md](./04-docker-compose-orchestration.md): orchestrating all services with Docker Compose.


