# ADR 0002: Database Strategy

## Context

Medical registry data (users, schedules, medical records) has strict relational dependencies. It is critical to avoid data anomalies, such as overlapping appointments or orphan clinical records.

## Decision

We chose **PostgreSQL** as the primary relational database, integrated via the **Diesel ORM**.

## Alternatives

- **MongoDB (NoSQL):** Considered for schema flexibility but rejected. A lack of built-in strict foreign key constraints increases the risk of data inconsistency.
- **SQLite:** Rejected due to limitations in handling highly concurrent write operations during peak booking times.

## Consequences

- **Pros:** Full ACID transaction support; strong data integrity guarantees through Foreign Keys and cascade operations; robust tooling and typing with Diesel.
- **Cons (Trade-offs):** Rigid schema structure; any changes to the data model require creating and applying strict SQL migrations.
