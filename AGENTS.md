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
- Open `docs/hostingplan.md` and `docs/hosting.md`. These files now define the active roadmap for the platform.
- Carefully review the code and documentation completed so far and ensure all previous work is truly implemented.
- Use the tasks and ideas outlined in `hostingplan.md` and `hosting.md` to determine what to build next.
- Implement each feature in full before considering it done.

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

- [ ] ✅ Consult `docs/hostingplan.md` and `docs/hosting.md` for the next feature or improvement.
- [ ] ✅ Verify that previously described functionality is actually implemented in code.
- [ ] ✅ Use the latest stable libraries and tools.
- [ ] ✅ Keep work modular, extensible, and production-capable.
- [ ] ✅ Write and update docs alongside real code.

---

> 🛑 **Never mark a step as complete until it has been implemented in real, runnable code.**

**Codex: You must refer to this AGENTS.md before every planning or code generation task.**
