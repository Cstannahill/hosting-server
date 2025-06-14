# AGENTS.md

## Guidance for Codex Agent Operations

**Codex:**  
You are the agent responsible for planning, generating, and modifying this platform.  
**Before starting any task, you must consult this document and follow its guidelines.**
Before planning or coding, open `docs/toc.md` and locate the first unchecked item.
Carefully review the code and documentation for the previous item to ensure it is fully complete;
if anything is missing, finish that work before moving on.

---

## Project Vision

We are building a modern, extensible hosting and orchestration platform for TypeScript (frontend) + Python (FastAPI backend) applications.  
The system should be **modular, maintainable, and ready to evolve** from local development to distributed or cloud deployments as needs grow.

---

## Core Principles

### 1. **Use Latest Stable Packages**

- Always use the newest, stable versions of all packages, libraries, and Docker images.
- Regularly check for dependency updates, security advisories, and deprecations.

### 2. **Best Practices and Modern Patterns**

- Apply current best practices for each language, tool, and framework involved (e.g., async for Rust, Vite for frontend, FastAPI patterns for Python).
- Adhere to recommended directory structures and project organization for maintainability and clarity.

### 3. **Modularity and Extensibility**

- Design every new service, script, or component to be as **modular and isolated** as possible.
- Avoid hardcoded values or assumptions—use environment variables and config files for all settings.
- Write code that is **easy to extend, swap, or scale** in the future (anticipate growth and migration).

### 4. **Isolation and Separation of Concerns**

- Ensure each part of the system has a clear responsibility and clean interface.
- Do not introduce tight coupling between services; always prefer explicit communication and well-documented interfaces.

### 5. **Documentation and Self-Explanation**

- Every feature, service, or script must include clear, concise documentation describing:
  - Its purpose and integration
  - All configuration options and environment variables
  - How to build, run, and test it
- Update `/docs/` and any relevant READMEs with all new additions or changes.

### 6. **Migration and Future-Proofing**

- Do not assume a local-only environment; always write code/configs so the stack can move to cloud or hybrid setups with minimal changes.
- Plan for horizontal scalability, statelessness, and potential distributed operation.

### 7. **Continuous Improvement**

- Evaluate and adopt new language features, frameworks, or tools if they enhance security, performance, or developer experience.
- Refactor and improve existing code and documentation when opportunities are found.

---

## Codex Agent: Checklist Before Every Task

- [ ] Consult the **latest documentation** and this AGENTS.md before beginning.
- [ ] Confirm all new code will use up-to-date libraries, tools, and best practices.
- [ ] Ensure your work is modular, isolated, and extensible.
- [ ] Update or generate documentation for all changes.
- [ ] Anticipate future migration, scaling, and extensibility needs.

---

> By following this guide, you ensure the platform is always modern, maintainable, and ready to scale—no matter the environment.

---

**Codex: You must refer to this AGENTS.md before every planning or code generation task.**
