use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::database::schema::{patient, employee, doctor, registry, medical_record, record_entry, appointment};

// ----------------------------------------
// PATIENT
// ----------------------------------------
#[derive(Queryable, Selectable, Identifiable, Debug, Clone)]
#[diesel(table_name = patient)]
#[diesel(primary_key(patient_id))]
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
    pub full_name: &'a str,
    pub birth_date: Option<NaiveDateTime>,
    pub phone: Option<&'a str>,
    pub address: Option<&'a str>,
}

// ----------------------------------------
// EMPLOYEE
// ----------------------------------------
#[derive(Queryable, Selectable, Identifiable, Debug, Clone)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = employee)]
#[diesel(primary_key(employee_id))]
pub struct Employee {
    pub employee_id: i32, // PK, never null
    pub role: String,
    pub full_name: String, // NOT NULL
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = employee)]
pub struct NewEmployee<'a> {
    pub role: &'a str,
    pub full_name: &'a str,
    pub phone: Option<&'a str>,
    pub email: Option<&'a str>,
}

// ----------------------------------------
// DOCTOR
// ----------------------------------------
#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Clone)]
#[diesel(table_name = doctor)]
#[diesel(belongs_to(Employee, foreign_key = employee_id))]
#[diesel(primary_key(employee_id))]
pub struct Doctor {
    pub employee_id: i32,  // PK, never null
    pub specialty: String, // NOT NULL
    pub office: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = doctor)]
pub struct NewDoctor<'a> {
    pub employee_id: i32,
    pub specialty: &'a str,
    pub office: Option<&'a str>,
}

// ----------------------------------------
// REGISTRY
// ----------------------------------------
#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Clone)]
#[diesel(table_name = registry)]
#[diesel(belongs_to(Employee, foreign_key = employee_id))]
#[diesel(primary_key(employee_id))]
pub struct RegistryWorker {
    pub employee_id: i32, // PK, never null
    pub window_number: Option<f64>,
}

#[derive(Insertable)]
#[diesel(table_name = registry)]
pub struct NewRegistryWorker {
    pub employee_id: i32, // Included! Not autoincremented here; inherited from Employee
    pub window_number: Option<f64>,
}

// ----------------------------------------
// MEDICAL RECORD
// ----------------------------------------
#[derive(Queryable, Selectable, Insertable, Clone, Debug)]
#[diesel(table_name = medical_record)]
pub struct MedicalRecord {
    pub record_number: i32, // PK, never null
    pub creation_date: Option<NaiveDateTime>,
    pub patient_id: i32, // NOT NULL
}

#[derive(Insertable)]
#[diesel(table_name = medical_record)]
pub struct NewMedicalRecord {
    pub creation_date: Option<NaiveDateTime>,
    pub patient_id: i32,
}

// ----------------------------------------
// RECORD ENTRY
// ----------------------------------------
#[derive(Queryable, Selectable, Insertable, Clone, Debug)]
#[diesel(table_name = record_entry)]
pub struct RecordEntry {
    pub entry_id: i32,
    pub record_number: i32,
    pub entry_number: Option<f64>,
    pub entry_date: Option<NaiveDateTime>,
    pub diagnosis: Option<String>,
    pub complaints: Option<String>,
}

#[derive(Insertable, Clone, Debug)]
#[diesel(table_name = record_entry)]
pub struct NewRecordEntry {
    pub record_number: i32,
    pub entry_number: Option<f64>,
    pub entry_date: Option<NaiveDateTime>,
    pub diagnosis: Option<String>,
    pub complaints: Option<String>,
}

// ----------------------------------------
// APPOINTMENT
// ----------------------------------------
#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Clone)]
#[diesel(belongs_to(Patient, foreign_key = patient_id))]
#[diesel(belongs_to(Doctor, foreign_key = doctor_id))]
#[diesel(belongs_to(RegistryWorker, foreign_key = registry_id))]
#[diesel(table_name = appointment)]
#[diesel(primary_key(appointment_id))]
pub struct Appointment {
    pub appointment_id: i32,
    pub appointment_date: Option<NaiveDateTime>,
    pub appointment_time: Option<String>,
    pub patient_id: i32,
    pub doctor_id: i32,
    pub registry_id: i32,
    pub status: Option<String>,
    pub reason: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = appointment)]
pub struct NewAppointment<'a> {
    pub appointment_date: Option<NaiveDateTime>,
    pub appointment_time: Option<&'a str>,
    pub patient_id: i32,
    pub doctor_id: i32,
    pub registry_id: i32,
    pub status: Option<&'a str>,
    pub reason: Option<&'a str>,
}
