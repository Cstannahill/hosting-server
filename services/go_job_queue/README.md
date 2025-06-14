# Go Job Queue Service

This service exposes a simple HTTP API to enqueue jobs and retrieve processed results.
Jobs are processed concurrently by multiple workers using goroutines and a shared channel.

## Endpoints
- `POST /enqueue` with JSON `{ "data": "text" }` returns the job ID.
- `GET /result?id=<ID>` returns the processed result or `202` if not ready.

The number of workers and queue size are minimal for demonstration.
