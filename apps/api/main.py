from fastapi import FastAPI
from fastapi.middleware.gzip import GZipMiddleware
import os
import uvicorn

app = FastAPI()
app.add_middleware(GZipMiddleware, minimum_size=1000)


@app.get("/")
async def read_root():
    env = os.environ.get("ENV", "development")
    return {"message": f"Hello from {env}"}

@app.get("/health")
async def health():
    return {"status": "ok"}


if __name__ == "__main__":
    port = int(os.environ.get("PORT", "8000"))
    uvicorn.run("main:app", host="0.0.0.0", port=port)
