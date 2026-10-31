# Session Handoff

## 2026-10-31

### Commit snapshot

- At the user's request, prepared the existing backend and documentation changes for a work-in-progress commit without completing or altering application code.
- Snapshot includes initial migration/Diesel schema alignment, `NaiveDate` patient birth dates, removal of scaffold routes, `/health_check`, and the patient DTO draft, plus project instructions, architecture documents, roadmap, and proposed API contract.
- `cargo check -p backend --offline` fails with E0425: `ValidationErrors` is undefined in `backend/src/database/dto.rs`. Its `validate_and_convert` body is empty, DTOs are not wired into handlers, and `PatientResponse` currently lacks `full_name`. The DTO work remains unfinished; the earlier passing build does not describe this snapshot.
- `git diff --check` reports existing trailing whitespace on the initial migration's appointment timestamp lines. No database migration or runtime tests were run in this session.

## 2026-10-28

### Agreed learning approach

- The user wants to build from small to big: basic patient CRUD first, then validation, consistent errors, accounts/JWT, and authorization. The new learning sequence in `5_implementation_plan.md` takes priority over the earlier next-task recommendation. API design now records this staged approach.
- Immediate next task: finish patient creation returning the saved database row with its generated ID, then add get-by-ID; proceed to list/update/delete one endpoint at a time. Delete is a local exercise for disposable patients without related data, not permission to cascade-delete clinical history or add production hard deletion.
- Preserve separate API DTOs (`CreatePatientRequest`, `PatientResponse`) and Diesel types (`NewPatient`, `Patient`). Initially use straightforward conversions; defer custom validation until CRUD works. Keeping `NewPatient` does not itself guarantee validity while its fields are public.
- Deferred validation design: plain Rust, with `ValidationErrors { fields: BTreeMap<String, Vec<String>> }`, deriving `Debug`, `Default`, `Serialize`, and methods `add` and `is_empty`. Collect field errors, return `Result<NewPatient, ValidationErrors>`, and validate before database work. Explicit handler mapping does not require `std::error::Error` or Actix `ResponseError`; later integrate with the shared application error.
- Deferred rules: trim names (1–150 characters); trim optional phone/address, convert blanks to `None`, enforce 20/200-character limits; reject future birth dates. Count characters, not bytes. The example used UTC today; prefer configured clinic date when implemented. Field violations return 422; malformed JSON/date representations and unknown fields return sanitized 400 responses. `serde_json::json!` requires a direct dependency if used.
- This session provided explanations and updated planning documents only. Validation examples were not implemented or tested. Existing uncommitted application changes were preserved; no fresh build or runtime verification was performed.

## 2026-10-21

### Baseline review

- `cargo check -p backend` passed, with one unused-import warning in `database/models.rs`. The working tree now uses `NaiveDate` for both patient birth-date fields; the earlier compilation/birth-date issue is resolved.
- Existing uncommitted application changes were preserved. No application code was changed during this review.
- Phase 1 still needs health/readiness routes, request/response DTOs and validation, consistent sanitized JSON errors, configuration validation, and an initial OpenAPI contract. A replay of the current migration files on a disposable database has not been verified in this session; this is not evidence that the user has never performed it.
- Patient creation currently returns the submitted fields without the generated patient ID or Location header and does not create a medical record. Patient listing has no pagination or stable ordering; its `Option<Vec<_>>`/404 branch is unnecessary because a successful Diesel collection load returns an empty vector when there are no rows.
- Patient routes have no authentication/authorization yet. Accounts, sessions, slots, and booking protection remain planned work.
- `pg_isready` found no response on the default local PostgreSQL socket. This does not establish whether the configured database or Docker database is reachable. No database schema inspection, migration execution, or runtime HTTP verification was performed.

### Follow-up database inspection

- Confirmed `postgres-container` is running and healthy. Read-only queries against its `clinic_db` show both Diesel migrations recorded as applied on 2026-10-08, including `202609201813530000`.
- Inspected public table columns, types, nullability, defaults, and constraints. These match the current initial migration in the inspected properties, including patient birth dates, nullable registry attribution, online-booking flag, and record-entry appointment linkage. No drift was found in those properties. This was not a full automated schema comparison or a fresh migration replay.
- The earlier recommendation overstated migration work as missing: the database is already migrated and the inspected schema agrees with the migration. Only fresh replay of the current files remains unverified by the assistant. The default socket probe was insufficient for assessing the Docker database.

### Suggested next task

Migration verification is marked complete at the user's direction on 2026-10-21. Verification evidence and its limits are recorded above; no disposable-database replay was performed by the assistant. Next: patient request/response DTOs and validation, followed by consistent errors, health/readiness, configuration checks, and the initial OpenAPI contract. Do not repeat the already-passing compilation fix.

## 2026-10-20

### Work completed

- Reviewed recent Git changes. The latest commit inspected was `6e744e7`, splitting the project into backend and desktop workspace packages and introducing the Actix backend. The commit and architecture ADR mention Tauri, but the desktop implementation still uses Iced.
- Reviewed all existing architecture documents, requirements, quality scenarios, and ADRs. Identified gaps between proposed architecture and current implementation, booking concurrency guarantees, measurable performance targets, record lifecycle, and authentication decisions.
- Explained the distinction between password verification (authentication), session management, and authorization.
- Updated `docs/adrs/0003-security-framework.md` to include Argon2id password hashing with unique random salts, encoded hash storage, password verification over HTTPS, JWT usage, and resource-level authorization. Added alternatives and trade-offs.
- Created `docs/5_implementation_plan.md`, an ordered roadmap with completion criteria.
- Created `docs/6_api_design.md`, a proposed `/api/v1` contract with methods, paths, permissions, examples, status codes, pagination, and current-route migration guidance.
- Added root `AGENTS.md` at the user's request to remind them to read the implementation plan and API design at the start of every project session.

### Implementation order

1. Fix backend compilation, model/schema mismatches, and migration consistency.
2. Implement accounts, Argon2id, sessions, and authorization.
3. Implement patient/staff profiles and doctor directory.
4. Implement doctor availability slots.
5. Implement transactional appointment booking, cancellation, and rescheduling.
6. Implement clinical records with ownership checks and internal-note filtering.
7. Connect the desktop client after confirming Iced versus Tauri.
8. Validate deployment, backup recovery, and quality targets.

First milestone: a patient can register, log in, select a doctor slot, book, and view/cancel the appointment.

### Proposed choices still requiring implementation decisions

- Fixed 30-minute slots are a roadmap assumption, not a confirmed business requirement.
- Admin handles reception initially; a separate Receptionist role is deferred.
- The plan proposes short-lived access JWTs and rotating opaque refresh tokens, with session/account checks for immediate revocation. This refines the existing stateless JWT ADR and must be recorded there before implementation.
- Define token lifetimes, secure client storage, staff provisioning, booking cutoffs, assigned-care access scope, and record correction/retention rules before enabling the corresponding flows.
- Patients see their own permitted clinical data; internal notes are excluded. Admin does not automatically receive clinical access.

### Code state and verification limits

- No application code was changed during this session. Documentation changes were checked for whitespace; no build, migration, or application tests were run.
- Existing local changes were present in the initial SQL migration, generated Diesel schema, and `backend/src/main.rs`. The docs directory was untracked at the last status check; nothing was staged or committed by the assistant.
- Known mismatch: patient models still use `NaiveDateTime` for birth dates while the schema uses `Date`; use `NaiveDate` when implementing the baseline fix.
- Existing applied databases will not receive edits to an already-applied initial migration automatically. Inspect schema drift and use forward migrations for retained databases.
- The appointment table currently has no slot-based double-booking protection. Accounts and availability are planned features, not existing implementations.
- A repository-wide whitespace check found pre-existing trailing spaces in the initial migration around the appointment timestamp fields. Those were left untouched.

### Suggested next task

Start with phase 1 in the implementation plan: inspect the current working tree, run `cargo check -p backend`, fix model/schema mismatches, and verify migrations against a disposable PostgreSQL database. Do not begin broad endpoint implementation before this baseline works.
