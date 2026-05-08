// HERE WILL BE ALL DATABASE MAGIC THINGS
use crate::database::models::{Appointment, Employee, NewEmployee, NewPatient, Patient};
use crate::database::schema::{employee, patient};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use tokio;

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
            .map_err(|e| format!("Database insertion failed: {}", e))
    })
    .await
    .map_err(|e| format!("Task panicked: {}", e))?
}

pub fn fetch_employees_db() -> Vec<Employee> {
    use crate::database::schema::employee::dsl::*;

    let connection = &mut establish_connection();

    employee
        .limit(15)
        .select(Employee::as_select())
        .load(connection)
        .expect("Error loading employee")
}

pub async fn update_employee_db(
    target_id: i32,
    new_role: String,
    new_full_name: String,
    new_phone: String,
    new_email: String,
) -> Result<Employee, String> {
    tokio::task::spawn_blocking(move || {
        use crate::database::schema::employee::dsl::*;

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
            .map_err(|e| format!("Database update failed: {}", e))
    })
    .await
    .map_err(|e| format!("Task panicked: {}", e))?
}
pub async fn delete_employee(target_id: i32) -> Result<usize, String> {
    // Push the heavy database work to a background thread
    use crate::database::schema::employee::dsl::*;
    tokio::task::spawn_blocking(move || {
        // Establish the connection
        let conn = &mut establish_connection();

        // Run your exact delete query, but use map_err instead of expect
        // so it doesn't crash the whole app if the database is locked
        diesel::delete(employee.filter(employee_id.eq(target_id)))
            .execute(conn)
            .map_err(|e| format!("Error deleting employee: {}", e))
    })
    .await
    .map_err(|e| format!("Error deleting employee: {}", e))?
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
            .map_err(|e| format!("Database insertion failed: {}", e))
    })
    .await
    .map_err(|e| format!("Task panicked: {}", e))?
}

pub async fn edit_patient_db(
    target_id: i32,
    new_name: String,
    new_phone: String,
    new_address: String,
    new_birth_date: Option<NaiveDateTime>,
) -> Result<Patient, String> {
    use crate::database::schema::patient::dsl::*;

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
            .map_err(|e| format!("Database update failed: {}", e))
    })
    .await
    .map_err(|e| format!("Task panicked: {}", e))?
}

pub async fn delete_patient_db(target_id: i32) -> Result<usize, String> {
    use crate::database::schema::patient::dsl::*;
    tokio::task::spawn_blocking(move || {
        let conn = &mut establish_connection();

        // Run your exact delete query, but use map_err instead of expect
        // so it doesn't crash the whole app if the database is locked
        diesel::delete(patient.filter(patient_id.eq(target_id)))
            .execute(conn)
            .map_err(|e| format!("Error deleting patient: {}", e))
    })
    .await
    .map_err(|e| format!("Task Paniced: {}", e))?
}

pub async fn insert_appointment_db(
    patient_id: i32,
    doctor_id: i32,
    registry_id: i32,
    appointment_date: Option<NaiveDateTime>,
    appointment_time: String,
    status: String,
    reason: String,
) -> Result<Appointment, String> {
}
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
