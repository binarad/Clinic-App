// HERE WILL BE ALL DATABASE MAGIC THINGS
use crate::database::models::{
    Appointment, Doctor, Employee, MedicalRecord, NewAppointment, NewDoctor, NewEmployee,
    NewPatient, NewRecordEntry, NewRegistryWorker, Patient, RecordEntry, RegistryWorker,
};
use crate::database::schema::{
    appointment, doctor, employee, medical_record, patient, record_entry, registry,
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use tokio;

pub struct AppointmentPayload {
    pub patient_id: i32,
    pub doctor_id: i32,
    pub registry_id: i32,
    pub date: Option<NaiveDateTime>,
    pub time: String,
    pub status: String,
    pub reason: String,
}

pub async fn insert_employee_db(
    role: String,
    full_name: String,
    phone: String,
    email: String,
) -> Result<Employee, String> {
    tokio::task::spawn_blocking(move || {
        let mut conn = establish_connection();

        // Format optional fields (empty strings become None)
        let phone_opt = if phone.trim().is_empty() {
            None
        } else {
            Some(phone.as_str())
        };
        let email_opt = if email.trim().is_empty() {
            None
        } else {
            Some(email.as_str())
        };

        let new_emp = NewEmployee {
            role: &role,
            full_name: &full_name,
            phone: phone_opt,
            email: email_opt,
        };

        // Insert and immediately return the newly created row!
        diesel::insert_into(employee::table)
            .values(&new_emp)
            .returning(Employee::as_select())
            .get_result(&mut conn)
            .map_err(|e| format!("Database insertion failed: {e}"))
    })
    .await
    .map_err(|e| format!("Task panicked: {e}"))?
}

pub async fn insert_doctor_db(
    full_name: String,
    phone: String,
    email: String,
    specialty: String,
    office: String,
) -> Result<(Employee, Doctor), String> {
    tokio::task::spawn_blocking(move || {
        let mut conn = establish_connection();

        let phone_opt = if phone.trim().is_empty() {
            None
        } else {
            Some(phone.as_str())
        };
        let email_opt = if email.trim().is_empty() {
            None
        } else {
            Some(email.as_str())
        };
        let office_opt = if office.trim().is_empty() {
            None
        } else {
            Some(office.as_str())
        };

        // Start Transaction
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            let new_emp = NewEmployee {
                full_name: &full_name,
                phone: phone_opt,
                role: "Doctor",
                email: email_opt,
            };

            let inserted_emp: Employee = diesel::insert_into(employee::table)
                .values(&new_emp)
                .returning(Employee::as_returning())
                .get_result(conn)?;

            // Grap the auto-generatd employee_id and insert the Doctor
            let new_doc = NewDoctor {
                employee_id: inserted_emp.employee_id,
                specialty: &specialty,
                office: office_opt,
            };

            let inserted_doc: Doctor = diesel::insert_into(doctor::table)
                .values(&new_doc)
                .returning(Doctor::as_returning())
                .get_result(conn)?;

            Ok((inserted_emp, inserted_doc))
        })
        .map_err(|e| format!("Transaction failed: {e}"))
    })
    .await
    .map_err(|e| format!("Task panicked: {e}"))?
}

pub async fn insert_registry_worker_db(
    full_name: String,
    phone: String,
    email: String,
    window_number: Option<f64>,
) -> Result<(Employee, RegistryWorker), String> {
    tokio::task::spawn_blocking(move || {
        let mut conn = establish_connection();

        // Format optional fields
        let phone_opt = if phone.trim().is_empty() {
            None
        } else {
            Some(phone.as_str())
        };
        let email_opt = if email.trim().is_empty() {
            None
        } else {
            Some(email.as_str())
        };

        // START TRANSACTION
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            // 1. Insert the Base Employee
            let new_emp = NewEmployee {
                full_name: &full_name,
                phone: phone_opt,
                role: "Registry", // Hardcoded safely!
                email: email_opt,
            };

            let inserted_emp: Employee = diesel::insert_into(employee::table)
                .values(&new_emp)
                .returning(Employee::as_returning())
                .get_result(conn)?;

            // 2. Grab the auto-generated employee_id and insert the Registry Worker
            let new_reg = NewRegistryWorker {
                employee_id: inserted_emp.employee_id, // Safely linked!
                window_number,
            };

            let inserted_reg: RegistryWorker = diesel::insert_into(registry::table)
                .values(&new_reg)
                .returning(RegistryWorker::as_returning())
                .get_result(conn)?;

            // 3. Return both structs
            Ok((inserted_emp, inserted_reg))
        })
        .map_err(|e| format!("Transaction failed: {e}"))
    })
    .await
    .map_err(|e| format!("Task panicked: {e}"))?
}

#[must_use] 
pub fn fetch_all_employees_db() -> Vec<Employee> {
    use crate::database::schema::employee::dsl::employee;

    let connection = &mut establish_connection();

    employee
        .select(Employee::as_select())
        .load(connection)
        .expect("Error loading employee")
}

#[must_use] 
pub fn fetch_doctors_joined_db() -> Vec<(Employee, Doctor)> {
    let mut conn = establish_connection();

    employee::table
        .inner_join(doctor::table)
        .select((Employee::as_select(), Doctor::as_select()))
        .load::<(Employee, Doctor)>(&mut conn)
        .unwrap_or_default()
}

/// Fetches all Registry Workers joined with their Base Employee data
#[must_use] 
pub fn fetch_registry_joined_db() -> Vec<(Employee, RegistryWorker)> {
    let mut conn = establish_connection();

    employee::table
        .inner_join(registry::table)
        // TELL DIESEL EXACTLY HOW TO MAP THE COLUMNS:
        .select((Employee::as_select(), RegistryWorker::as_select()))
        .load::<(Employee, RegistryWorker)>(&mut conn)
        .unwrap_or_default()
}

pub async fn update_employee_db(
    target_id: i32,
    new_role: String,
    new_full_name: String,
    new_phone: String,
    new_email: String,
) -> Result<Employee, String> {
    tokio::task::spawn_blocking(move || {
        use crate::database::schema::employee::dsl::{employee, employee_id, role, full_name, phone, email};

        let conn = &mut establish_connection();

        // Format optional fields
        let phone_opt = if new_phone.trim().is_empty() {
            None
        } else {
            Some(new_phone.as_str())
        };
        let email_opt = if new_email.trim().is_empty() {
            None
        } else {
            Some(new_email.as_str())
        };

        // Run the update query
        diesel::update(employee.filter(employee_id.eq(target_id)))
            .set((
                role.eq(&new_role),
                full_name.eq(&new_full_name),
                phone.eq(phone_opt),
                email.eq(email_opt),
            ))
            .returning(Employee::as_select()) // Return the newly updated row
            .get_result(conn)
            .map_err(|e| format!("Database update failed: {e}"))
    })
    .await
    .map_err(|e| format!("Task panicked: {e}"))?
}
pub async fn delete_employee(target_id: i32) -> Result<usize, String> {
    tokio::task::spawn_blocking(move || {
        let mut conn = establish_connection();

        // START TRANSACTION
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            // Using explicit absolute paths so Rust doesn't get confused by shadowed imports!

            // 1. Delete any appointments where this employee was the Doctor
            diesel::delete(
                crate::database::schema::appointment::table
                    .filter(crate::database::schema::appointment::doctor_id.eq(target_id)),
            )
            .execute(conn)?;

            // 2. Delete any appointments where this employee was the Registry Worker
            diesel::delete(
                crate::database::schema::appointment::table
                    .filter(crate::database::schema::appointment::registry_id.eq(target_id)),
            )
            .execute(conn)?;

            // 3. Delete their specific Subtype records
            diesel::delete(
                crate::database::schema::doctor::table
                    .filter(crate::database::schema::doctor::employee_id.eq(target_id)),
            )
            .execute(conn)?;

            diesel::delete(
                crate::database::schema::registry::table
                    .filter(crate::database::schema::registry::employee_id.eq(target_id)),
            )
            .execute(conn)?;

            // 4. Finally, delete the Base Employee record
            diesel::delete(
                crate::database::schema::employee::table
                    .filter(crate::database::schema::employee::employee_id.eq(target_id)),
            )
            .execute(conn)
        })
        .map_err(|e| format!("Cascade delete failed: {e}"))
    })
    .await
    .map_err(|e| format!("Task panicked: {e}"))?
}

#[must_use] 
pub fn fetch_patient_db() -> Vec<Patient> {
    use crate::database::schema::patient::dsl::patient;

    let connection = &mut establish_connection();

    patient
        .select(Patient::as_select())
        .load(connection)
        .expect("Error loading patients")
}

pub async fn insert_patient_db(
    full_name: String,
    phone: String,
    address: String,
    birth_date: Option<NaiveDateTime>,
) -> Result<Patient, String> {
    tokio::task::spawn_blocking(move || {
        let mut conn = establish_connection();

        // Format optional fields ( Empty strings become None)
        let phone_opt = if phone.trim().is_empty() {
            None
        } else {
            Some(phone.as_str())
        };

        let address_opt = if address.trim().is_empty() {
            None
        } else {
            Some(address.as_str())
        };

        let new_patient = NewPatient {
            full_name: &full_name,
            phone: phone_opt,
            address: address_opt,
            birth_date,
        };

        // Insert and immediately return the newly created row
        diesel::insert_into(patient::table)
            .values(&new_patient)
            .returning(Patient::as_returning())
            .get_result(&mut conn)
            .map_err(|e| format!("Database insertion failed: {e}"))
    })
    .await
    .map_err(|e| format!("Task panicked: {e}"))?
}

pub async fn edit_patient_db(
    target_id: i32,
    new_name: String,
    new_phone: String,
    new_address: String,
    new_birth_date: Option<NaiveDateTime>,
) -> Result<Patient, String> {
    use crate::database::schema::patient::dsl::{patient, patient_id, full_name, phone, address, birth_date};

    tokio::task::spawn_blocking(move || {
        let mut conn = establish_connection();

        let phone_opt = if new_phone.trim().is_empty() {
            None
        } else {
            Some(new_phone.as_str())
        };

        let address_opt = if new_address.trim().is_empty() {
            None
        } else {
            Some(new_address.as_str())
        };

        diesel::update(patient.filter(patient_id.eq(target_id)))
            .set((
                full_name.eq(&new_name),
                phone.eq(phone_opt),
                address.eq(address_opt),
                birth_date.eq(new_birth_date),
            ))
            .returning(Patient::as_returning())
            .get_result(&mut conn)
            .map_err(|e| format!("Database update failed: {e}"))
    })
    .await
    .map_err(|e| format!("Task panicked: {e}"))?
}

/// Deletes a Patient and all of their associated relational data safely
pub async fn delete_patient_db(target_id: i32) -> Result<usize, String> {
    tokio::task::spawn_blocking(move || {
        let mut conn = establish_connection();

        // START TRANSACTION
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            // 1. Delete all appointments assigned to this patient
            diesel::delete(
                crate::database::schema::appointment::table
                    .filter(crate::database::schema::appointment::patient_id.eq(target_id)),
            )
            .execute(conn)?;

            // A. Find the patient's medical records to get their IDs
            let record_ids: Vec<i32> = crate::database::schema::medical_record::table
                .filter(crate::database::schema::medical_record::patient_id.eq(target_id))
                .select(crate::database::schema::medical_record::record_number)
                .load::<i32>(conn)?;

            // B. Delete all entries inside those records
            diesel::delete(
                crate::database::schema::record_entry::table.filter(
                    crate::database::schema::record_entry::record_number.eq_any(&record_ids),
                ),
            )
            .execute(conn)?;

            // C. Delete the medical records themselves
            diesel::delete(
                crate::database::schema::medical_record::table
                    .filter(crate::database::schema::medical_record::patient_id.eq(target_id)),
            )
            .execute(conn)?;

            // 2. Finally, delete the Base Patient record
            diesel::delete(
                crate::database::schema::patient::table
                    .filter(crate::database::schema::patient::patient_id.eq(target_id)),
            )
            .execute(conn)
        })
        .map_err(|e| format!("Cascade delete failed: {e}"))
    })
    .await
    .map_err(|e| format!("Task panicked: {e}"))?
}

pub async fn insert_appointment_db(payload: AppointmentPayload) -> Result<Appointment, String> {
    tokio::task::spawn_blocking(move || {
        let mut conn = establish_connection();

        let time_opt = if payload.time.trim().is_empty() {
            None
        } else {
            Some(payload.time.as_str())
        };

        let status_opt = if payload.status.trim().is_empty() {
            None
        } else {
            Some(payload.status.as_str())
        };

        let reason_opt = if payload.reason.trim().is_empty() {
            None
        } else {
            Some(payload.reason.as_str())
        };

        let new_apt = NewAppointment {
            appointment_date: payload.date,
            appointment_time: time_opt,
            patient_id: payload.patient_id,
            doctor_id: payload.doctor_id,
            registry_id: payload.registry_id,
            status: status_opt,
            reason: reason_opt,
        };

        diesel::insert_into(appointment::table)
            .values(&new_apt)
            .returning(Appointment::as_returning())
            .get_result(&mut conn)
            .map_err(|e| format!("Database insertion failed: {e}"))
    })
    .await
    .map_err(|e| format!("Task panicked: {e}"))?
}

pub async fn update_appointment_db(
    target_id: i32,
    payload: AppointmentPayload,
) -> Result<Appointment, String> {
    tokio::task::spawn_blocking(move || {
        use crate::database::schema::appointment::dsl::{appointment, appointment_id, patient_id, doctor_id, registry_id, appointment_date, appointment_time, status, reason};
        let mut conn = establish_connection();

        // Convert empty UI strings into SQL NULLs (using .clone() since we own the strings here)
        let time_opt = if payload.time.trim().is_empty() {
            None
        } else {
            Some(payload.time.clone())
        };
        let status_opt = if payload.status.trim().is_empty() {
            None
        } else {
            Some(payload.status.clone())
        };
        let reason_opt = if payload.reason.trim().is_empty() {
            None
        } else {
            Some(payload.reason.clone())
        };

        // Update specific columns matching the target ID
        diesel::update(appointment.filter(appointment_id.eq(target_id)))
            .set((
                patient_id.eq(payload.patient_id),
                doctor_id.eq(payload.doctor_id),
                registry_id.eq(payload.registry_id),
                appointment_date.eq(payload.date),
                appointment_time.eq(time_opt),
                status.eq(status_opt),
                reason.eq(reason_opt),
            ))
            .returning(Appointment::as_returning())
            .get_result(&mut conn)
            .map_err(|e| format!("Database update failed: {e}"))
    })
    .await
    .map_err(|e| format!("Task panicked: {e}"))?
}

#[must_use] 
pub fn fetch_appointments_db() -> Vec<Appointment> {
    use crate::database::schema::appointment::dsl::appointment;
    let mut conn = establish_connection();

    appointment
        .load::<Appointment>(&mut conn)
        .unwrap_or_else(|_| vec![]) // Return an empty vector if the table is empty/fails
}

pub async fn delete_appointment_db(target_id: i32) -> Result<usize, String> {
    tokio::task::spawn_blocking(move || {
        use crate::database::schema::appointment::dsl::{appointment, appointment_id};
        let mut conn = establish_connection();

        diesel::delete(appointment.filter(appointment_id.eq(target_id)))
            .execute(&mut conn)
            .map_err(|e| format!("Error deleting appointment: {e}"))
    })
    .await
    .map_err(|e| format!("Task panicked: {e}"))?
}

// ==========================================
// MEDICAL RECORD OPERATIONS
// ==========================================

// Fetches all base medical records
#[must_use] 
pub fn fetch_all_medical_records_db() -> Vec<MedicalRecord> {
    let mut conn = establish_connection();
    medical_record::table
        .load::<MedicalRecord>(&mut conn)
        .unwrap_or_else(|_| vec![])
}

// Creates a new empty Medical record for a Patient (Useful if they don't have one yet)
pub async fn insert_medical_record_db(patient_id: i32) -> Result<MedicalRecord, String> {
    tokio::task::spawn_blocking(move || {
        let mut conn = establish_connection();
        let now = chrono::Local::now().naive_local();

        diesel::insert_into(medical_record::table)
            .values((
                medical_record::patient_id.eq(patient_id),
                medical_record::creation_date.eq(now),
            ))
            .get_result::<MedicalRecord>(&mut conn)
            .map_err(|e| format!("Failed to create medical record: {e}"))
    })
    .await
    .map_err(|e| format!("Task panicked: {e}"))?
}

// ==========================================
// RECORD ENTRY OPERATIONS
// ==========================================

// Fetches only the entries for a specific medical record
#[must_use] 
pub fn fetch_entries_for_record_db(target_record_number: i32) -> Vec<RecordEntry> {
    let mut conn = establish_connection();

    record_entry::table
        .filter(record_entry::record_number.eq(target_record_number))
        .order(record_entry::entry_date.desc())
        .load::<RecordEntry>(&mut conn)
        .unwrap_or_else(|_| vec![])
}

// Adds a new historical entry to a medical record
pub async fn insert_record_entry_db(payload: NewRecordEntry) -> Result<RecordEntry, String> {
    tokio::task::spawn_blocking(move || {
        let mut conn = establish_connection();

        diesel::insert_into(record_entry::table)
            .values(&payload)
            .get_result::<RecordEntry>(&mut conn)
            .map_err(|e| format!("Failed to insert record entry: {e}"))
    })
    .await
    .map_err(|e| format!("Task panicked: {e}"))?
}

// Updates an existing record entry
pub async fn update_record_entry_db(
    target_entry_id: i32,
    new_diagnosis: String,
    new_complaints: String,
) -> Result<RecordEntry, String> {
    tokio::task::spawn_blocking(move || {
        use crate::database::schema::record_entry::dsl::{record_entry, entry_id, diagnosis, complaints};
        let mut conn = establish_connection();

        let diag_opt = if new_diagnosis.trim().is_empty() {
            None
        } else {
            Some(new_diagnosis)
        };
        let comp_opt = if new_complaints.trim().is_empty() {
            None
        } else {
            Some(new_complaints)
        };

        diesel::update(record_entry.filter(entry_id.eq(target_entry_id)))
            .set((diagnosis.eq(diag_opt), complaints.eq(comp_opt)))
            .returning(RecordEntry::as_returning())
            .get_result(&mut conn)
            .map_err(|e| format!("Failed to update entry: {e}"))
    })
    .await
    .map_err(|e| format!("Task panicked: {e}"))?
}

// Safely deletes a specific record entry
pub async fn delete_record_entry_db(target_entry_id: i32) -> Result<usize, String> {
    tokio::task::spawn_blocking(move || {
        let mut conn = establish_connection();

        diesel::delete(record_entry::table.filter(record_entry::entry_id.eq(target_entry_id)))
            .execute(&mut conn)
            .map_err(|e| format!("Failed to delete entry: {e}"))
    })
    .await
    .map_err(|e| format!("Task panicked: {e}"))?
}

#[must_use] 
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}
