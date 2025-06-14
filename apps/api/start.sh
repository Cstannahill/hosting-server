#!/bin/bash
set -e

/app/prestart.sh

export PORT=${PORT:-8000}
export WORKERS=${WORKERS:-1}
export LOG_LEVEL=${LOG_LEVEL:-info}

exec uvicorn main:app --host 0.0.0.0 --port "$PORT" --workers "$WORKERS" --log-level "$LOG_LEVEL"
