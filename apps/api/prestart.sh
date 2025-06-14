#!/bin/bash
set -e

if [ -f /app/prestart.py ]; then
  echo "Running prestart tasks..."
  python /app/prestart.py
fi

