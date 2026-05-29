# Takdir 

**Takdir** (from the Arabic/South Asian root for *Orchestration* or *Destiny*) is a lightweight, remote-control API layer built to sit on top of your local AI harness. 

It allows you to dynamically manage and trigger complex, multi-step **skills** and **tools** on your powerful local machine from any remote device. Whether you want to kick off a heavy journey-planning script while on the move, generate markdown summaries, or manage your agentic workflows, Takdir bridges the gap between lightweight remote trigger and heavy local execution.

## 🚀 Features

* **Remote Orchestration:** Execute heavy local AI skills seamlessly from a lightweight remote client.
* ** Dynamic Skill Management:** Full CRUD capabilities (`POST`, `PUT`, `DELETE`) to upload and update your custom AI skill definitions remotely.
* **Remote Triggering:** Kick off heavy local AI workflows instantly from any remote device or terminal.
* **Status Tracking:** Built-in job queuing and polling so you can monitor exactly where your AI is in its execution pipeline.

## 🛠️ API Endpoints

### Skill Management
* `GET /skills` — List all currently available skills and tools.
* `POST /skills` — Upload a new skill payload or tool definition.
* `PUT /skills/:id` — Replace or update an existing skill.
* `DELETE /skills/:id` — Safely remove a skill from the local harness.

### Execution & Monitoring
* `POST /execute/:id` — Trigger a specific skill. Returns a unique `job_id` immediately.
* `GET /jobs/:job_id` — Check the real-time status of a running skill (e.g., `pending`, `running`, `completed`, `failed`).
