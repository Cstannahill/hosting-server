from fastapi import FastAPI
import os

app = FastAPI()

MESSAGE = os.getenv("MESSAGE", "Hello")

@app.get("/health")
async def health():
    return {"status": "ok"}

@app.get("/")
async def read_root():
    env = os.environ.get("ENV", "development")
    return {"message": f"{MESSAGE} from {env}"}
