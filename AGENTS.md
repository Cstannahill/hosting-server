# AGENTS.md

## Guidance for Codex Agent Operations

**Codex:**  
You are the agent responsible for **building** this platform. Your role is not to passively plan or document, but to **actively construct** the system as you move through the checklist.

> ⚠️ You are expected to generate real, functional code and infrastructure based on each document step.  
> **Do not skip building the features described in each doc.**  
> Documentation without implementation is incomplete.

---

## Mandatory Build Protocol

Before starting any task:
- Open `docs/toc.md` and locate the first unchecked item.
- Carefully review the code and documentation for the **previous step** and ensure it has been **fully implemented** — not just marked off.
- **Then read the current step, and implement it in full.**
- Only mark a step as complete if the corresponding code or configuration is functioning and present.

---

## Project Vision

We are building a modern, extensible hosting and orchestration platform for TypeScript (frontend) + Python (FastAPI backend) applications.  
The system must be **fully functional, incrementally built**, and ready for real deployments.  

---

## Core Principles (No changes needed here)
...
(unchanged for brevity — see original content)

---

## Codex Agent: Checklist Before Every Task

- [ ] ✅ Read and implement the next unchecked step in `docs/toc.md`.
- [ ] ✅ Ensure the previous step is complete and code exists.
- [ ] ✅ Use the latest stable libraries and tools.
- [ ] ✅ Keep work modular, extensible, and production-capable.
- [ ] ✅ Write and update docs alongside real code.

---

> 🛑 **Never mark a step as complete until it has been implemented in real, runnable code.**

**Codex: You must refer to this AGENTS.md before every planning or code generation task.**
