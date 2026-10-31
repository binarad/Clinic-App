# Proposed API Design

Status: Proposed contract, not a list of existing routes. Implement in the order in [the implementation plan](5_implementation_plan.md).

Learning sequence agreed on 2026-10-28: implement basic local patient CRUD first, then validation, consistent errors, authentication, and authorization. The permissions and complete response conventions below remain the target contract, not current capabilities. A DELETE operation may be used only as a local learning exercise on disposable patients without related data; it is not part of the production patient API. Patient updates must define omitted-versus-null behavior explicitly. See the implementation plan for the incremental steps and deferred validation design.

## Conventions

- Base path: `/api/v1`. Use lowercase plural resource names and hyphens for multiword names. IDs in paths identify resources; filters belong in query parameters.
- `GET` reads, `POST` creates or performs an explicit operation, `PATCH` partially updates. Do not use `GET` for mutations. These path conventions are project choices; HTTP does not mandate plural nouns.
- Use `201 Created` with a `Location` header for created resources, `200 OK` for reads/updates with bodies, and `204 No Content` for successful operations without bodies.
- Empty collections return `200` with `items: []`. Missing individual resources return `404`.
- Use `400` for malformed requests, `401` for absent/invalid authentication, `403` for denied operations, `404` for inaccessible individual resources where existence must be concealed, `409` for state/uniqueness conflicts, `422` for invalid field values, and `429` for rate limits. Return `WWW-Authenticate: Bearer` with protected-resource 401 responses.
- Return a common error shape, for example `{"error":{"code":"slot_unavailable","message":"This slot is no longer available.","request_id":"..."}}`. Add field errors for validation. Never return SQL details or credentials.
- Paginate collections with `limit` (default 20, maximum 100) and `offset` (default 0), returning `{"items":[],"limit":20,"offset":0,"total":0}`. Use a stable sort with an ID tie-breaker. Specify endpoint-specific filters in OpenAPI.
- Use RFC 3339 timestamps with offsets, `YYYY-MM-DD` for birth dates, and half-open time ranges `[from, to)`. Require bounded time ranges for slot queries and reject unreasonably large ranges.
- Authorization applies to response fields and database queries, including list endpoints. A query parameter never expands the caller's permissions.
- Use DTOs separate from Diesel models. Reject unknown sensitive fields such as `role` in patient registration rather than silently accepting them.

HTTP behavior reference: [RFC 9110](https://www.rfc-editor.org/rfc/rfc9110.html). Partial update reference: [RFC 5789](https://www.rfc-editor.org/rfc/rfc5789.html).

## Health

| Method | Path | Purpose |
| :--- | :--- | :--- |
| GET | `/health/live` | Process is running; no database query. |
| GET | `/health/ready` | Database connectivity and required schema are ready; 503 otherwise. |

## Authentication and accounts — phase 2

All paths below are relative to `/api/v1`.

| Method | Path | Purpose / access |
| :--- | :--- | :--- |
| POST | `/auth/register` | Public Patient registration; creates account, patient profile, and medical record atomically. |
| POST | `/auth/login` | Verify credentials; return access JWT and refresh token. |
| POST | `/auth/refresh` | Exchange a valid refresh token for a rotated refresh token and new access JWT. |
| POST | `/auth/logout` | Revoke the current session; 204. |
| GET | `/me` | Current account ID, role, and linked profile IDs; authenticated. |
| POST | `/me/password-change` | Verify current password, set new hash, revoke all sessions; authenticated; 204. |
| POST | `/accounts` | Admin creates a staff login linked to an employee, with an explicitly allowed role. |
| PATCH | `/accounts/{account_id}` | Admin activates/deactivates an account. Restrict DTO to allowed fields; role changes are deferred. |

Registration accepts login email, password, full name, and optional patient profile fields. Login accepts email/password. Neither returns a password hash. Staff provisioning should use an invitation flow before wider rollout; for a local MVP, a temporary password must require change before clinical access and must not appear in logs.

Define the refresh-token transport and secure client storage in OpenAPI/client documentation. Never place credentials or tokens in URLs. Deactivation/logout must invalidate sessions according to phase 2; removing a token only from the client is insufficient.

## Patients and staff — phase 3

| Method | Path | Purpose / access |
| :--- | :--- | :--- |
| GET | `/me/patient-profile` | Patient reads own profile. |
| PATCH | `/me/patient-profile` | Patient updates permitted own profile fields. |
| GET | `/patients` | Admin searches patient directory; bounded search and pagination. |
| POST | `/patients` | Admin creates a patient profile and medical record without creating login credentials. |
| GET | `/patients/{patient_id}` | Admin, that Patient, or a Doctor with an assigned-care relationship. |
| PATCH | `/patients/{patient_id}` | Admin updates permitted administrative profile fields. |
| GET | `/employees` | Admin lists staff. |
| POST | `/employees` | Admin creates an employee and optional doctor profile in one transaction. |
| GET | `/employees/{employee_id}` | Admin reads staff details. |
| PATCH | `/employees/{employee_id}` | Admin updates staff details or deactivates staff. |
| GET | `/doctors` | Authenticated users browse professional doctor profiles; filter by specialty/name. |
| GET | `/doctors/{doctor_id}` | Authenticated users read a professional doctor profile. |
| PATCH | `/doctors/{doctor_id}` | Admin updates specialty/office and other permitted professional fields. |

For the existing schema, `doctor_id` in the API maps to the doctor's `employee_id`. Document this mapping rather than introducing another conflicting identifier.

No hard-delete endpoints for patients or staff in the initial MVP. Directory responses must not include passwords, internal employee fields, or clinical history. Assigned-care access means the doctor has an appointment with the patient; define its permitted time window and history scope before enabling access.

## Availability — phase 4

| Method | Path | Purpose / access |
| :--- | :--- | :--- |
| GET | `/doctors/{doctor_id}/slots?from=...&to=...` | Authenticated users retrieve bookable slots. |
| POST | `/doctors/{doctor_id}/slots` | Admin creates a bounded batch of fixed-duration slots; atomic batch, 201 with a Location pointing to the collection. |
| PATCH | `/slots/{slot_id}` | Admin changes open/blocked state; cannot block an occupied slot. |

Slot listings return slot ID, doctor ID, start/end, and availability; never the identity of a booked patient. Only Admin may request an additional availability filter to inspect unavailable slots. Booking uses slot ID, not a time string supplied independently of availability.

## Appointments — phase 5

| Method | Path | Purpose / access |
| :--- | :--- | :--- |
| POST | `/appointments` | Patient books for self; Admin may book for a specified patient. |
| GET | `/appointments?from=...&to=...&status=booked` | Patient sees own; Doctor sees assigned; Admin sees scheduling data. |
| GET | `/appointments/{appointment_id}` | Same ownership rules as listing. |
| PATCH | `/appointments/{appointment_id}` | Cancel, reschedule, or update permitted status/reason fields according to role and lifecycle. |

Self-booking request:

```json
{"slot_id": 42, "reason": "Consultation"}
```

Admin booking additionally requires `patient_id`. Patients must not supply another identity. Set booking actor and booking channel server-side. The legacy `is_online_booking` field needs a documented meaning; actor attribution is the authoritative audit information.

Cancellation request:

```json
{"status": "cancelled"}
```

Rescheduling request:

```json
{"slot_id": 57}
```

Patients may cancel/reschedule their own future booked appointments; Admin may manage bookings; the assigned Doctor may mark a due appointment completed or no-show. Define exact timing cutoffs as business rules. Reject unsupported transitions and ambiguous combinations (such as changing slot and completing simultaneously). Keep appointment identity/history on rescheduling. Never cancel by deleting the row.

Return `409` for occupied slots or invalid state transitions. Define `Idempotency-Key` retention and replay behavior for `POST /appointments` before enabling automatic client retries. Responses include the authoritative slot start/end and status. Do not put clinical entry contents in appointment listings.

## Medical records — phase 6

| Method | Path | Purpose / access |
| :--- | :--- | :--- |
| GET | `/patients/{patient_id}/medical-record` | Read record summary; own Patient or authorized treating Doctor. |
| GET | `/medical-records/{record_number}/entries` | Paginated permitted history with the same access policy. |
| GET | `/record-entries/{entry_id}` | Read permitted entry fields with the same access policy. |
| POST | `/appointments/{appointment_id}/record-entry` | Assigned Doctor creates the appointment's single clinical entry; 201 with Location `/api/v1/record-entries/{entry_id}`. |

The backend derives record number and patient from the appointment; the request includes clinical fields, not ownership IDs. Validate that the appointment patient matches the medical record patient. A duplicate entry returns 409. Patient responses exclude internal notes. Admin has no clinical access by default. Entries are immutable initially; defer amendment endpoints until author attribution, correction history, and permissions are specified.

## Current routes to migrate

| Current route | Proposed route / behavior |
| :--- | :--- |
| `GET /patients` | `GET /api/v1/patients`, Admin-only with pagination; return an empty list rather than 404 when no patients match. |
| `POST /patient` | `POST /api/v1/patients` for Admin-created profiles; public signup uses `/api/v1/auth/register`. |
| `POST /echo` | Remove from production routing after the development scaffold is no longer needed. |

Use Actix `web::scope("/api/v1")` and module-level route configuration. Keep authentication middleware and authorization checks active on all protected scopes. Capture actual DTO schemas, status codes, filters, and role restrictions in OpenAPI as routes are implemented.
