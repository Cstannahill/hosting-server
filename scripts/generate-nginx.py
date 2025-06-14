import yaml
import os
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

def main():
    apps = load_registry()

    with open(TEMPLATE_PATH) as f:
        template = Template(f.read())

    rendered = template.render(apps=apps)

    with open(OUTPUT_PATH, "w") as f:
        f.write(rendered)

    print(f"✅ nginx.conf updated with {len(apps)} app(s)")

if __name__ == "__main__":
    main()
