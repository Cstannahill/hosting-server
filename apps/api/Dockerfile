FROM python:3.12-slim

# System setup
RUN apt-get update && apt-get install -y curl && \
    curl -sSL https://install.python-poetry.org | python3 - && \
    ln -s ~/.local/bin/poetry /usr/local/bin/poetry

WORKDIR /app

# Copy lock + toml first for better cache use
COPY pyproject.toml poetry.lock ./

# Install dependencies
RUN poetry config virtualenvs.create false && \
    poetry install --no-root --no-interaction --no-ansi

# Copy source last
COPY . .

EXPOSE 8000

ENV WORKERS=1
CMD ["bash", "-c", "poetry run uvicorn main:app --host 0.0.0.0 --port 8000 --workers ${WORKERS}"]
