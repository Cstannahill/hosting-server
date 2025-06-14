import os
import time
import sqlite3
import requests
import ollama
import chromadb

DB_PATH = os.environ.get("SQLITE_PATH", "/data/metrics.sqlite")
METRICS_URL = os.environ.get("METRICS_URL", "http://metrics_exporter:9300/metrics")
CHROMA_PATH = os.environ.get("CHROMA_PATH", "/data/chroma")
COLLECTION_NAME = os.environ.get("CHROMA_COLLECTION", "metrics")
INTERVAL = int(os.environ.get("CAPTURE_INTERVAL_SECS", "60"))


def init_db():
    conn = sqlite3.connect(DB_PATH)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS metrics (id INTEGER PRIMARY KEY AUTOINCREMENT, ts TEXT, data TEXT)"
    )
    conn.commit()
    return conn


def capture_metrics(conn):
    resp = requests.get(METRICS_URL, timeout=10)
    resp.raise_for_status()
    ts = time.strftime("%Y-%m-%dT%H:%M:%S")
    conn.execute("INSERT INTO metrics (ts, data) VALUES (?, ?)", (ts, resp.text))
    conn.commit()


def embed_and_reset(conn, client):
    cur = conn.execute("SELECT id, data FROM metrics")
    rows = cur.fetchall()
    if not rows:
        return
    ids = []
    docs = []
    for row_id, data in rows:
        ids.append(str(row_id))
        docs.append(data)
    embeddings = [
        ollama.embeddings(model="nomic-embed-text:v1.5", prompt=d)["embedding"]
        for d in docs
    ]
    collection = client.get_or_create_collection(COLLECTION_NAME)
    collection.upsert(documents=docs, ids=ids, embeddings=embeddings)
    conn.execute("DELETE FROM metrics")
    conn.commit()


def main():
    conn = init_db()
    client = chromadb.PersistentClient(path=CHROMA_PATH)
    while True:
        try:
            capture_metrics(conn)
            embed_and_reset(conn, client)
        except Exception as exc:
            print(f"error: {exc}")
        time.sleep(INTERVAL)


if __name__ == "__main__":
    main()
