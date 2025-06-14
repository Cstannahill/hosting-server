# 21. Data Capture and Vector Embedding

## Purpose

This service collects metrics from the stack, stores them in SQLite, and then
embeds the records using Ollama's `nomic-embed-text:v1.5` model. Embeddings are
stored in a local Chroma database. After a successful flush the SQLite table is
cleared to avoid unbounded growth.

## Key Features

- Periodically fetches metrics from `metrics_exporter`.
- Persists raw metrics in a SQLite database.
- Generates embeddings via the running Ollama server.
- Stores embeddings in a persistent Chroma collection.
- Deletes processed rows after they are embedded.

## Docker Compose Integration

```yaml
  ollama:
    image: ollama/ollama:latest
    container_name: ollama
    ports:
      - "11434:11434"
    volumes:
      - ollama_data:/root/.ollama

  data_capture:
    build: ./services/data_capture
    container_name: data_capture
    depends_on:
      - metrics_exporter
      - ollama
    volumes:
      - ./data:/data
    environment:
      - SQLITE_PATH=/data/metrics.sqlite
      - CHROMA_PATH=/data/chroma
      - METRICS_URL=http://metrics_exporter:9300/metrics
      - CAPTURE_INTERVAL_SECS=60
      - CHROMA_COLLECTION=metrics
      - OLLAMA_HOST=http://ollama:11434
```

## Requirements

- Python 3
- `requests`, `chromadb`, `ollama` packages
- A running Ollama instance with the `nomic-embed-text:v1.5` model (provided by the `ollama` service)
```
ollama pull nomic-embed-text:v1.5
```

The Chroma database is created in the mounted volume path.
