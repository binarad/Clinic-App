use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::database::schema::*;

// ----------------------------------------
// PATIENT
// ----------------------------------------
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = patient)]
pub struct Patient {
    pub patient_id: i32,   // PK, never null
    pub full_name: String, // NOT NULL
    pub birth_date: Option<NaiveDateTime>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = patient)]
pub struct NewPatient<'a> {
    // patient_id omitted (AUTOINCREMENT)
    pub full_name: &'a str,
    pub birth_date: Option<NaiveDateTime>,
    pub phone: Option<&'a str>,
    pub address: Option<&'a str>,
}

// ----------------------------------------
// EMPLOYEE
// ----------------------------------------
#[derive(Queryable, Selectable, Debug)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = employee)]
pub struct Employee {
    pub employee_id: i32,  // PK, never null
    pub full_name: String, // NOT NULL
    pub phone: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = employee)]
pub struct NewEmployee<'a> {
    // employee_id omitted (AUTOINCREMENT)
    pub full_name: &'a str,
    pub phone: Option<&'a str>,
}

// ----------------------------------------
// DOCTOR
// ----------------------------------------
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = doctor)]
pub struct Doctor {
    pub employee_id: i32,  // PK, never null
    pub specialty: String, // NOT NULL
    pub office: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = doctor)]
pub struct NewDoctor<'a> {
    pub employee_id: i32, // Included! Not autoincremented here; inherited from Employee
    pub specialty: &'a str,
    pub office: Option<&'a str>,
}

// ----------------------------------------
// REGISTRY
// ----------------------------------------
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = registry)]
pub struct Registry {
    pub employee_id: i32, // PK, never null
    pub window_number: Option<f64>,
}

// Note: No `<'a>` lifetime needed here because there are no string references
#[derive(Insertable)]
#[diesel(table_name = registry)]
pub struct NewRegistry {
    pub employee_id: i32, // Included! Not autoincremented here; inherited from Employee
    pub window_number: Option<f64>,
}

// ----------------------------------------
// MEDICAL RECORD
// ----------------------------------------
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = medical_record)]
pub struct MedicalRecord {
    pub record_number: i32, // PK, never null
    pub creation_date: Option<NaiveDateTime>,
    pub patient_id: i32, // NOT NULL
}

#[derive(Insertable)]
#[diesel(table_name = medical_record)]
pub struct NewMedicalRecord {
    // record_number omitted (AUTOINCREMENT)
    pub creation_date: Option<NaiveDateTime>,
    pub patient_id: i32,
}

// ----------------------------------------
// RECORD ENTRY
// ----------------------------------------
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = record_entry)]
pub struct RecordEntry {
    pub entry_id: i32,      // PK, never null
    pub record_number: i32, // NOT NULL
    pub entry_number: Option<f64>,
    pub entry_date: Option<NaiveDateTime>,
    pub diagnosis: Option<String>,
    pub complaints: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = record_entry)]
pub struct NewRecordEntry<'a> {
    // entry_id omitted (AUTOINCREMENT)
    pub record_number: i32,
    pub entry_number: Option<f64>,
    pub entry_date: Option<NaiveDateTime>,
    pub diagnosis: Option<&'a str>,
    pub complaints: Option<&'a str>,
}

// ----------------------------------------
// APPOINTMENT
// ----------------------------------------
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = appointment)]
pub struct Appointment {
    pub appointment_id: i32, // PK, never null
    pub appointment_date: Option<NaiveDateTime>,
    pub appointment_time: Option<String>,
    pub patient_id: i32,  // NOT NULL
    pub doctor_id: i32,   // NOT NULL
    pub registry_id: i32, // NOT NULL
    pub status: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = appointment)]
pub struct NewAppointment<'a> {
    // appointment_id omitted (AUTOINCREMENT)
    pub appointment_date: Option<NaiveDateTime>,
    pub appointment_time: Option<&'a str>,
    pub patient_id: i32,
    pub doctor_id: i32,
    pub registry_id: i32,
    pub status: Option<&'a str>,
}
