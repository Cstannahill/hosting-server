from fastapi import FastAPI
import os

app = FastAPI()

@app.get("/")
async def read_root():
    env = os.environ.get("ENV", "development")
    return {"message": f"Hello from {env}"}
