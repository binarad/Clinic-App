# Quality Attribute Scenarios (QAS)

## QAS 1: Performance

- **Source:** Patient.
- **Stimulus:** Sends a request to view available doctors and their time slots.
- **Environment:** Peak load (e.g., Monday morning), 1000 concurrent requests per second.
- **Artifact:** Backend API (Actix Web) and PostgreSQL database.
- **Response:** The system executes the query and returns the JSON schedule list.
- **Response Measure:** 95% of requests (p95) are completed in ≤ 150 ms.

## QAS 2: Reliability

- **Source:** Doctor.
- **Stimulus:** Opens the application to check the daily schedule.
- **Environment:** Normal operation, occasional minor network instability.
- **Artifact:** The entire system infrastructure.
- **Response:** The system successfully processes the request without server crashes.
- **Response Measure:** Overall service availability is ≥ 99.9%.

## QAS 3: Security

- **Source:** Unauthorized user (Attacker).
- **Stimulus:** Attempts to brute-force a doctor's account password.
- **Environment:** Public internet.
- **Artifact:** Authentication module.
- **Response:** The system detects the failed attempts and temporarily blocks access.
- **Response Measure:** Account/IP is blocked for 15 minutes after 5 failed attempts.

## QAS 4: Maintainability

- **Source:** Developer.
- **Stimulus:** Adds a new "Doctor's Notes" field to the Appointment table.
- **Environment:** Development environment during a feature update.
- **Artifact:** Codebase (Actix Web) and Diesel ORM schema.
- **Response:** Developer runs a database migration and updates DTOs without breaking existing functionality.
- **Response Measure:** Time to implement the change is < 4 hours, test coverage remains ≥ 80%.

## QAS 5: Functional Suitability

- **Source:** Patient.
- **Stimulus:** Clicks "Book Appointment".
- **Environment:** Normal load, concurrent booking attempts for the same time slot.
- **Artifact:** Appointments module, PostgreSQL.
- **Response:** The system validates slot availability via a database transaction, saves the record, and returns a confirmation.
- **Response Measure:** 0% duplicate bookings for a single slot; data integrity is maintained.
