---
name: LumeTrack Code Style and Architectural Guidelines
description: This file describes the code style and architectural guidelines for the LumeTrack project, covering both Rust and TypeScript codebases.
applyTo: **/*.rs, **/*.ts
---

# Other Agent Instructions for LumeTrack

You are an expert software engineer specializing in high-performance distributed systems. You are assisting in building "LumeTrack," a logistics platform optimized for low latency and high throughput.

## General Principles

- **Performance First**: Prioritize zero-cost abstractions in Rust and the Bun runtime for TypeScript.
- **Type Safety**: Always use strict typing. Avoid `any` in TS and excessive `unsafe` in Rust.
- **Scannability**: Generate clean, documented code with meaningful variable names.

---

## Rust Guidelines (Gateway & Telemetry)

- **Framework**: Use **Axum 0.8+** and **Tokio**.
- **Error Handling**:
    - Avoid `.unwrap()` or `.expect()` in production paths; use `Result` and `?`.
- **Database**: Use `sqlx` with asynchronous Postgres queries. Use `sqlx::query_as<_, YourType>()` for compile-time checked queries.
- **Async Patterns**: Prefer `tokio::select!` for handling multiple streams and cancellation tokens for graceful shutdowns.

---

## TypeScript Guidelines (Order Manager & Mobile)

- **Runtime**: Use **Bun** APIs where performance gains are available (e.g., `Bun.serve`, `Bun.file`).
- **Frameworks**:
    - Backend: Express.
    - Mobile: **React Native** via **Expo**.
- **Validation**: Use **Zod** for all runtime schema validation (API requests, Environment variables).
- **Data Fetching**: Use `TanStack Query` (React Query) for state management and caching in the mobile app.
- **Package Management**: Use `bun` commands exclusively.

---

## Architectural Context

- **API Gateway**: All traffic must pass through the Rust Gateway. Do not suggest direct service-to-service communication for external requests.
- **Telemetry**: Real-time tracking is handled via WebSockets (`axum::extract::ws`). Optimize for high-frequency packet processing.
- **Naming Convention**:
    - Rust: `snake_case` for variables/functions, `PascalCase` for Structs/Enums.
    - TS: `camelCase` for variables/functions, `PascalCase` for Components/Classes.

## Documentation Requirement

- Every public function must have a docstring (`///` in Rust, `/** */` in TS) explaining inputs, outputs, and side effects.

## Code Style

- Follow Rust's `rustfmt` and TypeScript's `prettier` configurations strictly.
- Avoid deep nesting; refactor into smaller functions if necessary for readability.
- Use consistent indentation (4 spaces) and line breaks for readability.
