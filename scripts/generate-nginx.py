import argparse
import os
import subprocess

import yaml
from jinja2 import Template

REGISTRY_PATH = "compose/app-registry"
TEMPLATE_PATH = "nginx/nginx.conf.template"
OUTPUT_PATH = "nginx/nginx.conf"

def load_registry():
    apps = []
    for file in os.listdir(REGISTRY_PATH):
        if file.endswith(".yaml"):
            with open(os.path.join(REGISTRY_PATH, file)) as f:
                apps.append(yaml.safe_load(f))
    return apps

def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Generate nginx.conf from app registry"
    )
    parser.add_argument(
        "--reload",
        action="store_true",
        help="Reload the nginx container after updating the config",
    )
    return parser.parse_args()


def reload_nginx() -> None:
    try:
        subprocess.run(
            ["docker", "exec", "nginx", "nginx", "-s", "reload"], check=True
        )
        print("✅ NGINX reloaded")
    except Exception as exc:
        print(f"⚠️  Failed to reload NGINX: {exc}")


def main() -> None:
    args = parse_args()
    apps = load_registry()

    with open(TEMPLATE_PATH) as f:
        template = Template(f.read())

    rendered = template.render(apps=apps)

    with open(OUTPUT_PATH, "w") as f:
        f.write(rendered)

    print(f"✅ nginx.conf updated with {len(apps)} app(s)")

    if args.reload:
        reload_nginx()

if __name__ == "__main__":
    main()
