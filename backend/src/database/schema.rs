// @generated automatically by Diesel CLI.

diesel::table! {
    appointment (appointment_id) {
        appointment_id -> Int4,
        appointment_date -> Nullable<Timestamp>,
        #[max_length = 10]
        appointment_time -> Nullable<Varchar>,
        patient_id -> Int4,
        doctor_id -> Int4,
        registry_id -> Int4,
        #[max_length = 50]
        status -> Nullable<Varchar>,
        reason -> Nullable<Text>,
    }
}

diesel::table! {
    doctor (employee_id) {
        employee_id -> Int4,
        #[max_length = 100]
        specialty -> Varchar,
        #[max_length = 20]
        office -> Nullable<Varchar>,
    }
}

diesel::table! {
    employee (employee_id) {
        employee_id -> Int4,
        #[max_length = 150]
        full_name -> Varchar,
        #[max_length = 20]
        phone -> Nullable<Varchar>,
        role -> Text,
        email -> Nullable<Text>,
    }
}

diesel::table! {
    medical_record (record_number) {
        record_number -> Int4,
        creation_date -> Nullable<Timestamp>,
        patient_id -> Int4,
    }
}

diesel::table! {
    patient (patient_id) {
        patient_id -> Int4,
        #[max_length = 150]
        full_name -> Varchar,
        birth_date -> Nullable<Timestamp>,
        #[max_length = 20]
        phone -> Nullable<Varchar>,
        #[max_length = 200]
        address -> Nullable<Varchar>,
    }
}

diesel::table! {
    record_entry (entry_id) {
        entry_id -> Int4,
        record_number -> Int4,
        entry_number -> Nullable<Numeric>,
        entry_date -> Nullable<Timestamp>,
        #[max_length = 255]
        diagnosis -> Nullable<Varchar>,
        #[max_length = 1000]
        complaints -> Nullable<Varchar>,
    }
}

diesel::table! {
    registry (employee_id) {
        employee_id -> Int4,
        window_number -> Nullable<Numeric>,
    }
}

diesel::joinable!(appointment -> doctor (doctor_id));
diesel::joinable!(appointment -> patient (patient_id));
diesel::joinable!(appointment -> registry (registry_id));
diesel::joinable!(doctor -> employee (employee_id));
diesel::joinable!(medical_record -> patient (patient_id));
diesel::joinable!(record_entry -> medical_record (record_number));
diesel::joinable!(registry -> employee (employee_id));

diesel::allow_tables_to_appear_in_same_query!(
    appointment,
    doctor,
    employee,
    medical_record,
    patient,
    record_entry,
    registry,
);
