# LumeTrack 💡

**LumeTrack** is a high-performance, real-time logistics and delivery orchestration platform. It is engineered with a "Performance-First" mindset, utilizing a polyglot microservices architecture to handle high-frequency telemetry data with sub-millisecond latency.

## 🚀 The Vision

LumeTrack aims to bridge the gap between heavy-duty industrial logistics and consumer-grade ease of use. By leveraging **Rust** for data-intensive telemetry and **TypeScript/Bun** for rapid business logic, LumeTrack provides a scalable foundation for tracking thousands of concurrent deliveries in real-time.

## 🏗️ System Architecture

LumeTrack is built as a suite of decoupled microservices, each optimized for its specific domain:

1.  **API Gateway (Rust + Axum)**: The high-performance "front door." Handles request routing, authentication, and WebSocket protocol upgrades.
2.  **Order Manager (TypeScript + Bun)**: Orchestrates the business lifecycle. Handles order placement, payment state management, and driver assignments.
3.  **Telemetry Service (Rust + Axum)**: A specialized engine for processing 3-second GPS heartbeats via WebSockets, validated against a Redis cache.
4.  **Analytics Service (Rust + SQLx)**: The "Historian." Subscribes to Redis Pub/Sub to persist movement data into PostgreSQL (PostGIS) for trip summaries.
5.  **Search Service (Rust + Axum)**: Provides lightning-fast spatial queries to help drivers find nearby packages using geographic indexing.
6.  **Notification Service**: A dedicated worker for real-time alerts to users and drivers via WebSockets/Push.

## 🛠️ Tech Stack

### Backend & Infrastructure

- **Languages**: Rust, TypeScript
- **Runtimes**: Tokio (Rust), Bun (JS/TS)
- **Web Frameworks**: Axum (Rust), Express (TS)
- **Communication**: WebSockets (Real-time), REST (Management), Redis Pub/Sub (Inter-service)
- **Databases**: PostgreSQL (with PostGIS), Redis (Caching & Message Broker)
- **Persistence**: SQLx (Rust), Prisma/Drizzle (TS)

### Frontend (Mobile)

- **Framework**: React Native via Expo
- **State Management**: TanStack Query
- **Navigation**: Expo Router

## 🏁 User Stories

### 👤 The User

- Place orders with destination, cargo weight, and contact details.
- Pay upfront via a secure escrow-based system.
- Track deliveries in real-time on a live map.
- Receive instant notifications upon successful delivery.

### 🚚 The Delivery Person

- Search for available orders within a specific geographic range.
- Filter orders based on vehicle capacity and cargo weight.
- Execute deliveries with real-time GPS telemetry streaming.
- Receive payment automatically upon delivery confirmation.

## 🚦 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (Latest stable)
- [Bun](https://bun.sh/)
- [Docker](https://www.docker.com/) & Docker Compose
- [PostgreSQL](https://www.postgresql.org/) & [Redis](https://redis.io/)

### License

LumeTrack is licensed under the MIT License. See [LICENSE](LICENSE) for more details.

### Author

Kavinda Rathnayake - [GitHub](https://github.com/kavinda-100)
