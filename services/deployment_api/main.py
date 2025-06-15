import os
import subprocess
import yaml
from fastapi import FastAPI, HTTPException

REGISTRY_PATH = os.environ.get("REGISTRY_PATH", "/repo/compose/app-registry")

app = FastAPI(title="Deployment API")


def load_registry():
    apps = {}
    for file in os.listdir(REGISTRY_PATH):
        if file.endswith(".yaml"):
            with open(os.path.join(REGISTRY_PATH, file)) as f:
                data = yaml.safe_load(f)
                if compose := data.get("compose_file"):
                    apps[data["name"]] = compose
    return apps


@app.get("/apps")
def list_apps():
    return {"apps": list(load_registry().keys())}


@app.post("/deploy/{app_name}")
def deploy(app_name: str):
    compose_file = load_registry().get(app_name)
    if not compose_file:
        raise HTTPException(status_code=404, detail="Unknown app")
    subprocess.run(["docker", "compose", "-f", compose_file, "up", "-d", "--build"], check=True)
    return {"status": "deployed", "app": app_name}


@app.post("/stop/{app_name}")
def stop(app_name: str):
    compose_file = load_registry().get(app_name)
    if not compose_file:
        raise HTTPException(status_code=404, detail="Unknown app")
    subprocess.run(["docker", "compose", "-f", compose_file, "down"], check=True)
    return {"status": "stopped", "app": app_name}
