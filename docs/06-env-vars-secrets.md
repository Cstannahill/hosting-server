Excellent! Here’s a detailed, production-minded template for **`06-env-vars-secrets.md`** on managing environment variables and secrets.

---

````md
# 06. Environment Variables & Secrets Management

## Purpose

This document explains how to securely manage configuration values, secrets, and sensitive credentials for your apps using environment variables.  
Environment variables allow you to keep secrets and configuration outside of your codebase for greater security and flexibility.

---

## Why Use Environment Variables?

- **Security:** Keep secrets (DB passwords, API keys) out of your source code.
- **Configurability:** Easily swap values for different environments (dev, prod, staging).
- **Portability:** Docker and Compose make it simple to inject env vars into any container.
- **Best Practice:** Most cloud platforms expect and encourage env-based configuration.

---

## Where to Store Environment Variables

### **.env File at Project Root**

- Place all sensitive values in a `.env` file:
  ```env
  POSTGRES_USER=myuser
  POSTGRES_PASSWORD=supersecret
  POSTGRES_DB=mydb
  API_SECRET_KEY=myapisecret
````

* **Never commit real secrets to git!**
  Add `.env` to your `.gitignore`.

### **(Optional) Per-Service .env Files**

* You may use per-service `.env` files (e.g., `apps/api/.env`) if you want further separation.

---

## Using Environment Variables in Docker Compose

**Reference the `.env` file in your Compose services:**

```yaml
services:
  api:
    build: ./apps/api
    env_file:
      - .env
  db:
    image: postgres:16
    env_file:
      - .env
```

* All variables in `.env` will be available in the container at runtime.

---

## Using Env Vars in Your Apps

### **In Python (FastAPI):**

```python
import os

DB_USER = os.environ.get("POSTGRES_USER")
DB_PASS = os.environ.get("POSTGRES_PASSWORD")
```

### **In Node.js (Vite/React):**

* Vite exposes only variables prefixed with `VITE_` to the frontend!

  * Example in `.env`:

    ```env
    VITE_API_BASE_URL=/api
    ```
* Access in code:

  ```js
  const apiBase = import.meta.env.VITE_API_BASE_URL
  ```

---

## Keeping Secrets Safe

* **Add `.env` to `.gitignore`**
  Example:

  ```
  .env
  ```
* **For production:**
  Set environment variables directly on the server or CI/CD pipeline. Do **not** copy development `.env` with real secrets.

---

## Example `.env` Template

Include a safe template (but not real secrets!) in your repo as `.env.example`:

```env
POSTGRES_USER=your_db_user
POSTGRES_PASSWORD=your_db_password
POSTGRES_DB=your_db_name
API_SECRET_KEY=your_api_secret
VITE_API_BASE_URL=/api
```

---

## Troubleshooting

* **App can't find env var:**

  * Ensure `env_file` is set in Compose.
  * Restart your containers after changing `.env`.
* **Frontend can't access variable:**

  * Remember, Vite requires the `VITE_` prefix.
* **Secrets in git history:**

  * If you accidentally committed a secret, [rotate credentials](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/removing-sensitive-data-from-a-repository) and remove them from your git history.

---

## Further Reading

* [Twelve-Factor App: Config](https://12factor.net/config)
* [Docker Compose: Environment Variables](https://docs.docker.com/compose/environment-variables/)

---

## Next Steps

* [ ] [07-persistent-volumes.md](./07-persistent-volumes.md): Persistent storage with Docker volumes.
* [ ] [08-networking-service-discovery.md](./08-networking-service-discovery.md): Internal networking and service discovery.

---

```

---

**How to use:**  
- Copy into `docs/06-env-vars-secrets.md`
- Add any project-specific secrets, naming conventions, or `.env.example` variables as your stack grows.

Ready for **step 7 (Persistent Volumes)**?
```
