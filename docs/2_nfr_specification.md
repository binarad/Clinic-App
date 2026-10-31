# Non-Functional Requirements (NFR) Specification

Based on the ISO/IEC 25010 standard, the following quality characteristics are defined for Clinic-App:

1. **Performance Efficiency**
   - *Requirement:* Fast retrieval of available doctor time slots.
   - *Metric:* API response latency p95 ≤ 150 ms at 1000 RPS.

2. **Reliability**
   - *Requirement:* High availability of the scheduling system.
   - *Metric:* System availability (Uptime) ≥ 99.9% per month.

3. **Security**
   - *Requirement:* Protection against brute-force attacks on medical staff accounts.
   - *Metric:* Temporary IP/account lockout for 15 minutes after 5 consecutive failed login attempts; 100% of API traffic uses TLS.

4. **Maintainability**
   - *Requirement:* Safe implementation of business logic changes.
   - *Metric:* Unit test coverage for core business logic (appointments module) ≥ 80%.

5. **Functional Suitability**
   - *Requirement:* Guaranteed preservation of booked appointments under heavy load.
   - *Metric:* 0% lost confirmed appointments; strict data consistency maintained via ACID transactions.
