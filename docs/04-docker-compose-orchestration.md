# 04. Docker Compose Orchestration

## Purpose

Docker Compose manages and orchestrates all services in the platform – frontend, backend, database, and reverse proxy – using a single command.

## Why Docker Compose?

- **Unified management:** Start and stop every service together.
- **Networking:** Containers communicate over an isolated network.
- **Declarative:** Service definitions live in a single YAML file.
- **Scalability:** Easy to add new services or scale existing ones.

## File Structure

The Compose file lives at the repository root:

```
my-hosting-platform/
├── docker-compose.yml
├── apps/
│   ├── web/
│   └── api/
├── nginx/
│   └── nginx.conf
├── services/
└── ...
```

## Example `docker-compose.yml`

```yaml
version: "3.8"

services:
  web:
    build: ./apps/web
    container_name: web
    ports:
      - "3000:3000"
    environment:
      - NODE_ENV=production

  api:
    build: ./apps/api
    container_name: api
    ports:
      - "8000:8000"
    environment:
      - ENV=production

  db:
    image: postgres:16
    container_name: db
    environment:
      POSTGRES_USER: myuser
      POSTGRES_PASSWORD: mypassword
      POSTGRES_DB: mydb
    volumes:
      - db_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"

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

volumes:
  db_data:
```

## Typical Commands

- **Start all services:**

  ```bash
  docker-compose up -d --build
  ```

- **Stop everything:**

  ```bash
  docker-compose down
  ```

- **View logs:**

  ```bash
  docker-compose logs -f
  ```

- **See container status:**

  ```bash
  docker-compose ps
  ```

## Best Practices

- Use `env_file:` or environment variables to keep secrets out of version control.
- Bind mount configs like `nginx.conf` for quick changes without rebuilding.
- Name your volumes for straightforward backup and migration.

## Troubleshooting

- **Port conflicts:** ensure only one service maps to a host port.
- **Build failures:** check Dockerfile paths and build context.
- **Database not persisting:** verify volume paths and permissions.

## Next Steps

- [x] [05-nginx-reverse-proxy.md](./05-nginx-reverse-proxy.md): setting up the reverse proxy for routing.


