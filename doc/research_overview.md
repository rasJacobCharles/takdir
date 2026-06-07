# 🔬 Technology Stack Research Overview

This document summarises the technology stack research and evaluations conducted for the **Takdir** project. The goal was to identify the stack providing the absolute highest performance and lowest latency for API requests while interfacing cleanly with local AI harnesses.

## 🎯 Core Verdict: Rust

Rust was chosen as the implementation language to guarantee:
- **Sub-millisecond response latency** for route handling.
- **Zero rubbish collection (GC) pauses**, ensuring extremely flat and predictable latency profiles (low jitter).
- **Concurrency safety** guaranteed at compile time via the borrow checker.
- **Minimal resource footprint** (typically running at < 15MB RAM idle).

---

## 📊 Component Comparisons

### 1. Web Frameworks: Axum vs. Actix-web
- **Actix-web:** Excels in raw microsecond-level single-threaded routing performance.
- **Axum:** Offers superior ergonomics, integrates natively with the `tower` middleware ecosystem, and uses `tokio` directly.
- **Verdict:** Axum was selected for its balance of high performance and standard community ergonomics.

### 2. Database Drivers: SQLx vs. tokio-postgres
- **SQLx:** Offers compile-time query verification but introduces runtime parsing and mapping overhead.
- **tokio-postgres:** Direct, low-level driver offering the absolute highest throughput and lowest latency.
- **Verdict:** Pair `tokio-postgres` with `deadpool-postgres` connection pooling for maximum database request performance.

### 3. Background Task execution: Apalis vs. tokio::spawn
- **tokio::spawn:** Lightweight and fast, but lacks persistence, retry logic, or external queue integration.
- **Apalis + Redis:** Provides robust, stateful background task queues, allowing the API server to respond immediately with a `202 Accepted` status.
- **Verdict:** Use `apalis-redis` for durable asynchronous task execution.

---

## 🚫 Scope Boundaries
- **API Focus Only:** This research is strictly limited to the API layer, database connections, and background task handoffs. Front-end architecture and local model weight orchestration (such as loading GGUF models directly in-process) are out of scope.
- **API-First LLM Calls:** The local harness interacts with large models via web APIs (Ollama, Gemini, OpenAI) rather than running in-process neural networks, making Rust's lack of native PyTorch bindings a non-issue.
