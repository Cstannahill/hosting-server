import os
import subprocess
import yaml

REGISTRY_PATH = "compose/app-registry"

def load_registry():
    apps = []
    for file in os.listdir(REGISTRY_PATH):
        if file.endswith(".yaml"):
            with open(os.path.join(REGISTRY_PATH, file)) as f:
                apps.append(yaml.safe_load(f))
    return apps

def run_compose(app, action):
    compose_file = app.get("compose_file")
    if not compose_file:
        return
    cmd = ["docker", "compose", "-f", compose_file, action, "-d"]
    subprocess.run(cmd, check=True)


def main():
    import sys
    action = sys.argv[1] if len(sys.argv) > 1 else "up"
    apps = load_registry()
    for app in apps:
        print(f"Running '{action}' for {app['name']}")
        run_compose(app, action)

if __name__ == "__main__":
    main()
