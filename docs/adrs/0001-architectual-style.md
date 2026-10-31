# ADR 0001: Architectural Style

## Context

The Clinic-App is developed by a small team (solo developer). The system requires a reliable, modern user interface for both doctors and patients, communicating with a centralized backend. The backend must have clear boundaries between domains (users, appointments) without turning into spaghetti code.

## Decision

We chose a Client-Server architecture utilizing a **Modular Monolith** pattern for the backend.
- **Client:** Desktop application built with Tauri (React, TypeScript).
- **Server:** A monolithic REST API built with Actix Web (Rust), logically separated into modules.

## Alternatives

- **Microservices:** Rejected due to unnecessary operational complexity and infrastructure overhead for a small-scale MVP.
- **Traditional Monolith (Server-Side Rendering):** Rejected because a modern, responsive desktop UI (Tauri) is required.

## Consequences

- **Pros:** Simpler to develop, test, and deploy as a single binary; fast in-memory communication between modules.
- **Cons (Trade-offs):** Individual modules cannot be scaled independently under extreme load scenarios.
