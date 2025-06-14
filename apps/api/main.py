from fastapi import FastAPI
from fastapi.middleware.gzip import GZipMiddleware
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import ORJSONResponse
from loguru import logger
import os
import sys
import uvicorn

LOG_LEVEL = os.getenv("LOG_LEVEL", "INFO").upper()
logger.remove()
logger.add(sys.stderr, level=LOG_LEVEL)

app = FastAPI(default_response_class=ORJSONResponse)
app.add_middleware(GZipMiddleware, minimum_size=1000)
app.add_middleware(
    CORSMiddleware,
    allow_origins=os.getenv("ALLOWED_ORIGINS", "*").split(","),
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.get("/")
async def read_root():
    env = os.environ.get("ENV", "development")
    logger.info("Root request")
    headers = {"Cache-Control": "public, max-age=60"}
    return ORJSONResponse({"message": f"Hello from {env}"}, headers=headers)

@app.get("/health")
async def health():
    headers = {"Cache-Control": "public, max-age=60"}
    return ORJSONResponse({"status": "ok"}, headers=headers)


if __name__ == "__main__":
    port = int(os.environ.get("PORT", "8000"))
    uvicorn.run("main:app", host="0.0.0.0", port=port)
