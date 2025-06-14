#!/bin/bash
set -e

echo "Updating repository..."
git pull

echo "Rebuilding and restarting containers..."
docker-compose up -d --build
