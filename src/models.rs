use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::schema::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = appointment)]
pub struct Appointment {
    pub appointment_id: Option<i32>,
    pub appointment_date: Option<NaiveDateTime>,
    pub appointment_time: Option<String>,
    pub patient_id: i32,
    pub doctor_id: i32,
    pub registry_id: i32,
    pub status: Option<String>,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = doctor)]
pub struct Doctor {
    pub employee_id: Option<i32>,
    pub specialty: String,
    pub office: Option<String>,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = employee)]
pub struct Employee {
    pub employee_id: Option<i32>,
    pub full_name: String,
    pub phone: Option<String>,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = medical_record)]
pub struct MedicalRecord {
    pub record_number: Option<i32>,
    pub creation_date: Option<NaiveDateTime>,
    pub patient_id: i32,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = patient)]
pub struct Patient {
    pub patient_id: Option<i32>,
    pub full_name: String,
    pub birth_date: Option<NaiveDateTime>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = registry)]
pub struct Registry {
    pub employee_id: Option<i32>,
    pub window_number: Option<f64>,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = record_entry)]
pub struct RecordEntry {
    pub entry_id: Option<i32>,
    pub record_number: i32,
    pub entry_number: Option<f64>,
    pub entry_date: Option<NaiveDateTime>,
    pub diagnosis: Option<String>,
    pub complaints: Option<String>,
}
