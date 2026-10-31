# ADR 0003: Security and Authentication

## Context

The system involves different user roles (Admin, Doctor, Patient). The Tauri client operates independently from the Actix backend. Passwords must be stored securely, and authenticated users must be able to make subsequent HTTP requests without repeatedly sending their passwords. The backend must enforce access permissions for each request.

## Decision

We chose **Argon2id password hashing** for password storage and verification, together with **JWT (JSON Web Tokens)** for authenticating subsequent API requests.

- **Password storage:** The Actix backend hashes each password using Argon2id and a unique, cryptographically random salt. The database stores only the encoded hash, including the salt and hashing parameters. Plaintext passwords must never be stored or logged. Hashing parameters must be benchmarked on the deployment hardware and meet the [OWASP password storage guidance](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html).
- **Login:** The client sends credentials over HTTPS. The backend verifies the supplied password against the stored Argon2id hash using the password hashing library's verification function. A successful login results in a signed JWT with a defined expiration time.
- **Subsequent requests:** The client sends the JWT in the HTTP Authorization header (Bearer Token). The backend validates its signature and expiration before accepting the authenticated identity.
- **Authorization:** The backend checks roles and resource-level permissions for each protected operation. For example, a Patient may access only their own medical records. Possessing a valid token does not by itself grant access to every resource.

## Alternatives

- **Plaintext or reversibly encrypted passwords:** Rejected for password storage; the application needs to verify passwords without recovering their original values.
- **General-purpose hashes (e.g., SHA-256 alone):** Rejected for password storage because they are too fast to provide adequate resistance to offline password guessing. Argon2id provides configurable memory and computation costs.
- **Server-side Sessions (Cookies):** Rejected because the application is built as a standalone desktop client (Tauri), and managing stateful sessions complicates API scalability.
- **OAuth2 / OpenID Connect:** Rejected for the MVP phase as it introduces unnecessary complexity for internal system authentication.

## Consequences

- **Pros:** Passwords are not recoverable through decryption; salted Argon2id hashes increase the cost of offline password guessing if the database is exposed. JWTs allow subsequent requests without resending passwords or requiring server-side session storage.
- **Cons (Trade-offs):** Argon2id intentionally consumes memory and CPU, so hashing parameters and login rate limits must account for available server capacity. Password hashing does not replace HTTPS, login rate limiting, or authorization checks. JWTs are difficult to revoke immediately (e.g., when permissions change) unless a revocation mechanism is implemented; token lifetime and revocation behavior must be defined before deployment.
