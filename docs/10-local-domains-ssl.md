Here’s a practical, modern template for **`10-local-domains-ssl.md`**—covering custom local domains and local HTTPS using SSL certificates.

---

```md
# 10. Local Domains & SSL (Optional, Advanced)

## Purpose

This guide explains how to set up custom local domains (e.g., `myapp.local`) and HTTPS (SSL/TLS) for your local hosting platform.  
This enables a more production-like experience, especially for testing subdomains, CORS, and secure cookies.

---

## Why Set Up Local Domains and SSL?

- **Realism:** Test your apps locally under the same domain patterns as production.
- **CORS/Security:** Ensure HTTPS-only features (e.g., secure cookies, OAuth, service workers) work as intended.
- **Developer Experience:** Avoid issues that only appear in HTTPS or multi-domain production environments.

---

## 1. Custom Local Domains

### **A. `/etc/hosts` Approach (Simple, Cross-Platform)**

1. **Edit your hosts file:**  
   Open `/etc/hosts` (Linux/macOS) or `C:\Windows\System32\drivers\etc\hosts` (Windows) as administrator.
2. **Add entries for your local domains:**  
```

127.0.0.1   myapp.local
127.0.0.1   api.myapp.local

````
3. **Access your services:**  
- Frontend: [http://myapp.local](http://myapp.local)
- API: [http://api.myapp.local](http://api.myapp.local) *(if mapped in NGINX)*

### **B. dnsmasq or Acrylic DNS (Advanced/Multiple Domains)**

- Tools like [dnsmasq](https://dnsmasq.io/) or [Acrylic DNS Proxy](https://mayakron.altervista.org/wikibase/show.php?id=AcrylicHome) let you wildcard map all `*.local` domains.
- Useful if you want any subdomain to just work without extra config.

---

## 2. Local HTTPS with mkcert

For local development, you can use [mkcert](https://github.com/FiloSottile/mkcert) to generate trusted SSL certificates.

### **A. Install mkcert**

- macOS:  
`brew install mkcert nss`
- Ubuntu/Linux:  
[See mkcert install instructions](https://github.com/FiloSottile/mkcert#installation)
- Windows:  
Download from [releases](https://github.com/FiloSottile/mkcert/releases) and add to PATH.

### **B. Generate Certificates**

```bash
mkcert myapp.local
mkcert api.myapp.local
````

* Produces `myapp.local.pem` and `myapp.local-key.pem`, etc.

### **C. Update NGINX to Use SSL**

```nginx
server {
    listen 443 ssl;
    server_name myapp.local;

    ssl_certificate     /etc/nginx/certs/myapp.local.pem;
    ssl_certificate_key /etc/nginx/certs/myapp.local-key.pem;

    location / {
        proxy_pass http://web:3000;
        ...
    }
}
```

* Mount your certs directory into the NGINX container via Docker Compose:

  ```yaml
  nginx:
    ...
    volumes:
      - ./nginx/nginx.conf:/etc/nginx/nginx.conf:ro
      - ./nginx/certs:/etc/nginx/certs:ro
  ```

### **D. Visit Your Secure Local App**

* Go to [https://myapp.local](https://myapp.local)
* Your browser should trust the mkcert CA automatically.

---

## 3. Routing Multiple Apps or Subdomains

* In NGINX, add more `server` blocks for each domain or use `location` blocks for paths.
* Example for subdomains:

  ```nginx
  server {
      listen 443 ssl;
      server_name api.myapp.local;
      ...
      location / {
          proxy_pass http://api:8000;
      }
  }
  ```

---

## Troubleshooting

* **Browser shows certificate warning:**
  Make sure mkcert’s CA is installed and the correct cert is being served.
* **Can’t access domain:**
  Double-check `/etc/hosts` entries and that NGINX is running.
* **Permission errors with certs:**
  Ensure Docker Compose mounts have the right permissions and paths.

---

## Security Note

* **Never use self-signed dev certs in production!**
  For live deployments, always use a real CA like [Let’s Encrypt](https://letsencrypt.org/).

---

## Further Reading

* [mkcert: Instant Local CA](https://github.com/FiloSottile/mkcert)
* [NGINX SSL docs](https://nginx.org/en/docs/http/configuring_https_servers.html)
* [Docker Compose and HTTPS](https://dev.to/kwirke/how-to-enable-https-for-local-development-with-docker-compose-and-mkcert-4p5h)

---

## Next Steps

* [ ] [11-monitoring-logs.md](./11-monitoring-logs.md): Add logging and basic monitoring for better visibility into your apps.

---

```

---

**How to use:**  
- Paste into `docs/10-local-domains-ssl.md`
- Adjust for your actual domain names, subdomains, and NGINX config structure as needed.

Ready for **step 11 (Monitoring & Logs, optional)?**
```
