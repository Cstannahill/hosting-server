# Extending the Hosting Platform for Go Backends

## Purpose

This document describes how the platform can be expanded to host Go-based APIs and services. Go's lightweight concurrency model opens opportunities for efficient background tasks and high-performance APIs. Below is a high-level design and implementation plan focused on leveraging goroutines and channels.

## Goals
- Add Go as a first-class backend option alongside Python, Node.js and C#.
- Provide examples and tooling for building, containerizing and deploying Go services.
- Demonstrate concurrency patterns using goroutines and channels.

## Features

1. **Channel-Based Job Queue**
   - Implement a lightweight job queue service written in Go.
   - Workers process jobs concurrently using goroutines and a shared channel.
   - Expose HTTP endpoints to enqueue jobs and retrieve results.

2. **Event Broadcast System**
   - Add a small event bus package where subscribers receive events through channels.
   - Services can publish events, and multiple goroutines can handle them in parallel.
   - Useful for log forwarding, metrics collection or real-time notifications.

3. **Concurrent Health Checks**
   - Provide a Go utility that periodically checks service health using goroutines.
   - Results are sent through channels to a central collector for reporting to the dashboard.
   - Allows fast parallel checks across many services.

4. **gRPC Support with Streaming**
   - Document how to scaffold a gRPC server in Go.
   - Include examples of streaming RPCs using goroutines to handle client streams.
   - gRPC services can run alongside HTTP APIs and be proxied by NGINX.

5. **CLI Tool for Background Workers**
   - Build a command-line tool in Go that launches background workers for tasks like data ingestion or scheduled jobs.
   - Uses channels to coordinate tasks and a context to manage graceful shutdown.
   - Containerize the tool so it can run as part of the platform's services.

## Implementation Outline

1. **Sample Go API**
   - Create `apps/go-api/` with a minimal Go HTTP server.
   - Dockerize it using a multi-stage build: `golang:1.21-alpine` for build, then a scratch or distroless image for runtime.
   - Provide a `docker-compose.yml` exposing a configurable `PORT`.
   - Add a registry entry under `compose/app-registry/` referencing this compose file.

2. **Concurrency Examples**
   - In `services/` add Go-based utilities (job queue, event bus, etc.).
   - Each service uses goroutines and channels heavily with clear README examples.

3. **Documentation & Tutorials**
   - Document how to run `go build` and `go test` within the monorepo.
   - Provide code snippets showing idiomatic channel usage (worker pools, fan-in/fan-out, etc.).
   - Update `docs/toc.md` with a new step for Go support when implementation starts.

4. **Integration with Existing Platform**
   - The `generate-nginx.py` script already reads registry entries. Once the Go API compose file is added, it will be proxied automatically.
   - Demonstrate launching the Go service using `scripts/launch_apps.py`.

5. **Future Enhancements**
   - Expand the job queue into a distributed task system using message queues.
   - Add examples of Go services interacting with the Rust metrics exporter.
   - Consider using Go's embed package to ship static assets alongside a Go API.

## Next Steps

- Develop the sample Go API and add its Dockerfile and compose file.
- Implement the job queue and event bus services.
- Update documentation throughout the platform to reference Go as a supported language.

