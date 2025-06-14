# 01. Project Overview & Goals

## Purpose
This document outlines the vision and overall objectives of the hosting platform. It serves as a starting point for contributors who need a high-level understanding of what the project aims to achieve.

## Vision
We are building a modern, extensible hosting and orchestration platform for a TypeScript frontend and a Python FastAPI backend. The system should be modular, maintainable, and ready to evolve from local development to distributed or cloud deployments.

## Key Goals
- **Consistency** across environments using Docker for all services.
- **Isolation** and clear separation of concerns between components.
- **Extensibility** so new services and features can be added with minimal friction.
- **Best practices** for security, scalability, and developer experience.
- **Comprehensive documentation** accompanying every part of the stack.

## Roadmap
The project is organized into incremental steps. After setting goals, you will:
1. Define the directory structure and initial setup.
2. Dockerize backend services and orchestrate them with Docker Compose.
3. Configure NGINX as a reverse proxy.
4. Manage environment variables and secrets securely.
5. Configure persistent volumes and service networking.
6. Establish monitoring, logging, scaling, and additional Rust utilities.

## Next Step
Continue to [02-directory-setup.md](./02-directory-setup.md) to learn about the monorepo layout and how to bootstrap the project.
