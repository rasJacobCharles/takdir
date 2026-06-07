# 🌌 Takdir

> **Takdir** (from the Arabic/South Asian root for *Orchestration* or *Destiny*) is a lightweight, remote-control API layer built to sit on top of a local AI harness.

It allows you to dynamically manage and trigger complex, multi-step **skills** and **tools** on a powerful local machine from any remote device. Whether you want to kick off a heavy journey-planning script while on the move, generate markdown summaries, or manage your agentic workflows, Takdir bridges the gap between a lightweight remote trigger and heavy local execution.

---

## 🎯 Project Goals

- **Microsecond API Routing Latency:** Sub-millisecond route handling and minimal CPU/RAM overhead on a Linux server.
- **Asynchronous Execution Pattern:** Immediately return `202 Accepted` with a unique Job ID upon execution trigger, offloading long-running model APIs to background workers.
- **Network-Isolated Sandboxing:** Run user-defined tools/scripts securely in a fully isolated Docker container sandbox with no network access.
- **Secure Access Control:** Strict JWT verification, rate-limiting on authentication endpoints, and Role-Based Access Control (RBAC).

---

## 🏗️ Architecture & Technology Stack

The project is implemented in **Rust** as a Cargo Workspace to ensure memory safety, concurrency safety, and zero rubbish-collection jitter.

- **`libs/takdir-core`:** Shared library for database models/mapping, password hashing, and token claims verification.
- **`bins/takdir-api`:** High-performance Web API server application.
- **`bins/takdir-admin`:** Administrative CLI tool to manage initial system state (e.g., seeding credentials).

*For a detailed breakdown of the components, database schema mapping, and runtime guarantees, see the [Architecture & Design Spec](doc/architecture_design.md).*

---

## 📚 Technical Documentation & Research

All technical evaluations and implementation roadmaps are kept inside this repository under the `doc/` directory:

- **[Technology Stack Research Overview](doc/research_overview.md):** Detailed analysis comparing Axum vs. Actix-web, SQLx vs. tokio-postgres, and Apalis vs. in-memory queues.
- **[Architecture & Design Spec](doc/architecture_design.md):** Workspace layout, security details (Docker sandboxing), and request flow sequence diagram.
- **[Project Roadmap & Task Breakdown](doc/roadmap.md):** Phase-by-phase implementation details covering active and future task items.

---

## 🛠️ Development Setup (Prerequisites)

Ensure the following are installed and running locally:
1. **Rust / Cargo** (MSRV 1.75+)
2. **PostgreSQL** (Running locally on default port `5432` or via local Unix socket)
3. **Redis** (Running locally on default port `6379`)
4. **Docker Daemon** (Ensure the docker service is active and the local user has permission to interact with it, e.g., via `bollard` over `/var/run/docker.sock`)

### Setup Command Cheat Sheet

```bash
# Verify services are running
docker ps
pg_isready
redis-cli ping

# Run tests (once workspace is initialised)
cargo test --all

# Start API server in development mode
cargo run --bin takdir-api

# Run admin CLI
cargo run --bin takdir-admin -- --help
```
