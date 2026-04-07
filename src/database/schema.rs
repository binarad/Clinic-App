// @generated automatically by Diesel CLI.

diesel::table! {
    appointment (appointment_id) {
        appointment_id -> Integer,
        appointment_date -> Nullable<Timestamp>,
        appointment_time -> Nullable<Text>,
        patient_id -> Integer,
        doctor_id -> Integer,
        registry_id -> Integer,
        status -> Nullable<Text>,
    }
}

diesel::table! {
    doctor (employee_id) {
        employee_id -> Integer,
        specialty -> Text,
        office -> Nullable<Text>,
    }
}

diesel::table! {
    employee (employee_id) {
        employee_id -> Integer,
        full_name -> Text,
        phone -> Nullable<Text>,
    }
}

diesel::table! {
    medical_record (record_number) {
        record_number -> Integer,
        creation_date -> Nullable<Timestamp>,
        patient_id -> Integer,
    }
}

diesel::table! {
    patient (patient_id) {
        patient_id -> Integer,
        full_name -> Text,
        birth_date -> Nullable<Timestamp>,
        phone -> Nullable<Text>,
        address -> Nullable<Text>,
    }
}

diesel::table! {
    record_entry (entry_id) {
        entry_id -> Integer,
        record_number -> Integer,
        entry_number -> Nullable<Double>,
        entry_date -> Nullable<Timestamp>,
        diagnosis -> Nullable<Text>,
        complaints -> Nullable<Text>,
    }
}

diesel::table! {
    registry (employee_id) {
        employee_id -> Integer,
        window_number -> Nullable<Double>,
    }
}

diesel::joinable!(appointment -> doctor (doctor_id));
diesel::joinable!(appointment -> patient (patient_id));
diesel::joinable!(appointment -> registry (registry_id));
diesel::joinable!(doctor -> employee (employee_id));
diesel::joinable!(medical_record -> patient (patient_id));
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
