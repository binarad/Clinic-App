# Implementation Plan

Status: Proposed roadmap, based on the current backend and ADRs. Endpoint contracts are in [API design](6_api_design.md). These documents do not describe implemented features.

## Current learning sequence — agreed 2026-10-28

Build the backend from small to big. The sequence below takes priority over the ordering of the broader phases below. Complete one patient endpoint at a time before adding cross-cutting features. These steps are planned, not implemented.

1. **Basic patient CRUD.** Finish create so it returns the saved row and generated ID; make list return an empty collection successfully; add get-by-ID, update, and delete. Use `/api/v1/patients` and `/api/v1/patients/{patient_id}`. Keep Diesel operations inside blocking execution. Start with simple request/response DTOs separate from database models, without custom validation. Verify persistence, missing-ID responses, and each operation manually against PostgreSQL. Define update semantics explicitly, including omitted versus null optional fields for PATCH. Delete is a local learning exercise using disposable patients without related data; do not use cascading deletion to remove clinical history or add hard deletion to the production contract.
2. **Validation.** Normalize input and add field rules to create/update. Implement `ValidationErrors` with `BTreeMap<String, Vec<String>>`, `add`, and `is_empty`; collect field errors before accessing the database. Plain Rust is sufficient initially. Use trimmed name length 1–150 characters, optional phone up to 20, optional address up to 200, blank optional strings converted to `None`, and no future birth dates. Count characters rather than bytes. Use the configured clinic date for the date comparison when available. Return 422 for field validation and 400 for malformed JSON or invalid date representations. Add focused validation tests.
3. **Consistent errors and API basics.** Centralize sanitized JSON errors, including JSON extraction failures; add pagination and stable ordering, configuration checks, health/readiness, and an initial OpenAPI contract. Maintain a simple written route contract during the CRUD exercise.
4. **Accounts and authentication.** Add accounts, Argon2id, registration, login, and access JWTs in small increments. Then implement the agreed refresh/session/revocation behavior, recording the design refinement in ADR 0003 first.
5. **Authorization.** Enforce roles and patient ownership on every relevant query and response; test cross-user access denial. The unauthenticated CRUD exercise is local development with synthetic data; permission enforcement is required before shared deployment or real patient use.
6. **Expand workflows.** Follow the detailed phases below for staff, availability, transactional booking, medical records, and the client. Add atomic patient/medical-record creation before enabling the final patient-creation workflow.

Immediate next task: finish patient creation returning the persisted `Patient` (including `patient_id`), then implement get-by-ID. Validation and `ValidationErrors` remain deferred learning steps.

## MVP assumptions

- One clinic, with Patient, Doctor, and Admin roles. Admin handles reception duties initially; add a Receptionist role later if needed.
- Self-registration creates Patient accounts only. Admin provisions staff accounts. Bootstrap the first Admin through a local administrative command, never public registration.
- Proposed booking model: fixed 30-minute slots aligned to a shared time grid. Confirm this assumption before implementing availability; variable-duration visits require overlap constraints instead.
- Store appointment instants as PostgreSQL `TIMESTAMPTZ`; accept and return RFC 3339 timestamps with offsets. Use the configured clinic timezone (`Europe/Kyiv` initially) for schedule entry and display. Birth dates remain `DATE`.
- Admin manages scheduling and staff but does not automatically receive access to clinical notes. Doctors access records for assigned appointments; patients see their own patient-visible clinical data. Internal doctor notes are excluded from patient responses.
- Deactivate accounts and cancel appointments rather than deleting history. Define clinical corrections and retention before enabling medical record editing.

## 1. Establish a working backend baseline

- Make `cargo check -p backend` pass. Align Diesel models with the schema, starting with `Patient` and `NewPatient` using `NaiveDate` for `birth_date`.
- [x] Migration verification — marked complete on 2026-10-21 at the user's direction. The running PostgreSQL database has both Diesel migrations applied, and inspected schema properties match the current migration. A disposable-database replay was not performed by the assistant. For future changes to retained databases, use forward migrations rather than editing applied history and check for schema drift.
- Organize code into `auth`, `patients`, `staff`, `scheduling`, and `medical_records` modules as each feature is implemented. Keep HTTP handlers, business rules, and database operations separate without adding empty abstractions.
- Introduce request/response DTOs, input validation, a shared error response, configuration validation, and `/health/live` and `/health/ready`.
- Keep synchronous Diesel queries and Argon2 work off Actix async workers using blocking execution.
- Start an OpenAPI contract with the first implemented routes; maintain it with each phase.

Done when: backend compiles, a fresh database migrates successfully, readiness detects database failure, and errors return consistent JSON without internal database details.

## 2. Add accounts, login, and permission enforcement

- Add an `accounts` table with a unique normalized login email, encoded password hash, explicitly assigned role, active flag, and timestamps. Link patient and employee profiles to accounts; staff-created patients may have no account yet.
- Keep login email on the account separate from optional profile contact fields. Do not automatically attach an account to an existing patient based only on a matching email; define a verified linking process.
- Implement Argon2id hashing and verification from ADR 0003. Public registration must not accept a role, employee ID, or existing patient ID.
- Create the patient profile and medical record atomically during patient registration. Also support patients created by Admin without login access.
- Implement login, current-user lookup, password change, login throttling, and generic invalid-credentials responses. Never return password hashes or log passwords/tokens.
- Define JWT issuer, audience, allowed signing algorithm, expiration, signing key configuration, and key rotation. Validate these claims on every protected request.
- Proposed session design: short-lived access JWTs plus rotating opaque refresh tokens. Store only refresh token hashes in session rows with expiry/revocation state. Rotate atomically and revoke the session on refresh-token reuse. Record this refinement in ADR 0003 before implementation.
- For immediate logout/deactivation, check session revocation and account active status on protected requests. This deliberately adds a database check; do not continue describing the complete authentication design as fully stateless.
- Revoke all sessions after a password change. Define token lifetimes and secure desktop token storage before wiring up the client. Do not persist passwords.
- Add authorization helpers for role and resource ownership. Scope list queries as well as item queries.

Done when: registration/login/refresh/logout work, revoked sessions are rejected, and tests prove patients cannot access another patient's data or choose a staff role.

## 3. Implement patient and staff profiles

- Implement patient list/detail/create/update and the current patient's profile.
- Implement employee creation/update/deactivation and doctor profiles. Admin may create a staff account through the account endpoint; link it to an employee profile without exposing credentials in profile responses.
- Implement doctor directory responses with public professional fields only.
- Deactivation must prevent new bookings and report any future appointments requiring reassignment; do not cascade-delete appointments or records.
- Replace destructive cascading deletion rules where they conflict with history retention.

Done when: Admin can provision a doctor and a patient can view available doctors, with contact/clinical fields filtered by permission.

## 4. Build availability before booking

- Add doctor slots with doctor ID, start/end instants, and open/blocked state. For the proposed fixed-grid MVP, enforce slot duration, grid alignment, and unique `(doctor_id, starts_at)` in the database.
- Admin creates slots over a bounded range or blocks an existing slot. Reject overlapping or invalid slots. Reject blocking a slot with an active booking until that booking is explicitly handled.
- Expose available slots filtered by date range. Use bounded queries and pagination.

Done when: a patient can retrieve genuinely bookable slots, and slot creation/blocking rules are tested against PostgreSQL.

## 5. Implement appointment booking and lifecycle

- Add `slot_id`, constrained status values, actor attribution, and timestamps to appointments. Derive doctor/time from the slot rather than trusting duplicate client fields.
- Add a partial unique index on `slot_id` for statuses that occupy the slot. Start with `booked`, `completed`, and `no_show`; only `cancelled` releases a slot. Define transitions explicitly: `booked -> cancelled | completed | no_show`; terminal states cannot be reopened through the generic update route.
- Book in a transaction: lock the slot, validate availability and active doctor/patient, insert the appointment, and commit before reporting success. Slot blocking must acquire the same lock. Map slot conflicts to HTTP 409.
- Patient identity comes from authentication for self-booking. Only Admin may supply a different patient ID. Record the actor on every booking/cancellation.
- Implement list/detail, cancellation, and staff status changes with ownership checks. For rescheduling, atomically move an existing `booked` appointment to a new slot, locking both slots in a consistent order; a failed move must preserve the original booking.
- Support a bounded, documented `Idempotency-Key` contract for booking retries, scoped to caller and request payload. Persist the key/result atomically with the booking; reuse with a different payload returns 409.

Done when: two simultaneous requests for the same slot yield one booking and one 409; retrying the same request does not create another appointment; failed rescheduling preserves the original booking; cancellation releases the slot.

## 6. Add clinical record workflows

- Implement medical record summaries and paginated entry lists.
- Create a record entry from an assigned appointment; derive patient and medical record from the appointment on the server. Enforce the existing one-entry-per-appointment rule.
- Separate patient-visible complaints/diagnosis/treatment from internal notes. Add a treatment field if treatment recording remains in scope.
- Record author and creation time. Keep entries immutable for the initial MVP; design audited amendments before allowing corrections. No entry deletion endpoint initially.
- Audit access and changes without copying clinical content into application logs.

Done when: an assigned doctor can record a visit, a patient can read their permitted history, and unrelated users and scheduling-only Admins cannot retrieve clinical content.

## 7. Connect the desktop client one workflow at a time

- Confirm the Iced-to-Tauri decision before investing in new UI. The current desktop package uses Iced while ADR 0001 specifies Tauri.
- Connect login, doctor search, slot selection, booking, and appointment history first; then doctor schedule and clinical entry workflows; then administration.
- Handle expired sessions, validation errors, conflicts, empty lists, and network retries explicitly. Keep permission enforcement on the backend.

Done when: Patient booking and Doctor visit recording work end to end against the API.

## 8. Validate deployment and quality targets

- Add PostgreSQL integration tests throughout phases 2–6, especially ownership, transactions, concurrency, and session revocation. Add end-to-end tests for the complete workflows once the client is connected.
- Configure HTTPS termination, secret management, production logging, migrations during deployment, and database backup/restore procedures.
- Demonstrate a restore and define acceptable recovery time/data loss. Verify committed appointments survive application restarts.
- Define dataset, workload, hardware, and measurement boundary before evaluating the documented 1000 RPS / p95 150 ms target. Treat it as a target until measured.
- Keep ADRs, OpenAPI, and quality scenarios aligned with what has actually shipped.

First milestone: a patient can register, log in, choose a doctor slot, book once, and see/cancel the appointment. Finish this before adding dashboards or broad administration features.
