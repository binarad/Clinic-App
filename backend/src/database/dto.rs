use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::database::models::NewPatient;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreatePatientRequest {
    pub full_name: String,
    pub birth_date: Option<NaiveDate>,
    pub phone: Option<String>,
    pub address: Option<String>
}

impl CreatePatientRequest {
    pub fn validate_and_convert(self) -> Result<NewPatient, ValidationErrors> {

    }
}
#[derive(Serialize)]
pub struct PatientResponse {
    pub patient_id: i32,
    pub birth_date: Option<NaiveDate>,
    pub phone: Option<String>,
    pub address: Option<String>
}
