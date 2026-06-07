# 🗺️ Project Roadmap & Work Breakdown

This document tracks the execution phases and planned tasks for the implementation of **Takdir**.

---

## 🚀 Phase 1: Initialise Codebase and Implement API (In Progress)

The goal of this phase is to build the core Rust API server, CLI setup, and asynchronous worker skeleton.

- **01: Project Setup & Stack Initialisation**
  - Configure the Cargo Workspace layout.
  - Setup async connection pools (`deadpool-postgres` & `bb8`/`deadpool-redis`).
  - Configure `apalis-redis` queue framework.
- **02: Skill CRUD Endpoints**
  - Implement endpoints: `GET /skills`, `POST /skills`, `PUT /skills/:id`, and `DELETE /skills/:id`.
  - Parse skill configs (YAML frontmatter + markdown body).
  - Add tool/script file uploads with a 5MB payload limit and `BYTEA` backup.
- **03: Job Execution & Monitoring Endpoints**
  - Implement `POST /execute/:id` and `GET /jobs/:job_id` endpoints.
  - Integrate Docker execution sandbox (`Bollard`) running under isolated networks (`--network none`).
  - Add startup watchdog to clean up orphaned Docker containers.
- **04: API Test Suite**
  - Write unit and integration test suite targeting endpoints, connection pools, and container isolations.
- **05: User Authentication & RBAC**
  - Implement endpoints: `POST /auth/register` and `POST /auth/login`.
  - Use `argon2id` for password hashing and generate HMAC-SHA256 JWT tokens.
  - Set up Redis denylisting for instant revocation/logout.

---

## 📋 Phase 2: Local AI Harness and Skill Integration (Todo)

This phase integrates the API with the actual local AI model runner and API key setup.

- **01: Harness Base Image**: Build the Docker container template containing standard tool runners.
- **02: External API Key Configurations**: Securely inject Gemini, OpenAI, Anthropic, or Ollama endpoint keys into the executor container.
- **03: Standard Skills Development**: Create and test core utility scripts (e.g., summary generators, scraper/journey planners).
- **04: Sandbox Verification**: Run end-to-end integration execution tests.

---

## 📋 Phase 3: Client-Side CLI or Remote Client (Todo)

This phase builds the client CLI for remote machines to interact with the Takdir API.

- **01: Authentication Flows**: Standardise JWT retrieval and local credential storage on remote clients.
- **02: Skill Management Command-line tools**: Add CLI commands to upload, update, and search skills.
- **03: Execution Triggers**: Develop shell command triggers to easily execute tasks and poll statuses.

---

## 📋 Phase 4: Production Infrastructure & Deployment (Todo)

This phase details how the server will be deployed on a production Linux machine.

- **01: Production Databases**: Configure replica setups, automated database backups, and socket permissions.
- **02: Nginx & SSL**: Setup reverse proxy configurations, TLS certificates (Let's Encrypt), and request headers.
- **03: Systemd Configuration**: Setup persistent daemons for `takdir-api` and background workers.
