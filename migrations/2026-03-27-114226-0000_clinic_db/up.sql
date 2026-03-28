-- Your SQL goes here
CREATE TABLE patient (
    patient_id INTEGER PRIMARY KEY AUTOINCREMENT,
    full_name VARCHAR(150) NOT NULL,
    birth_date DATETIME,
    phone VARCHAR(20),
    address VARCHAR(200)
);

CREATE TABLE employee (
    employee_id INTEGER PRIMARY KEY AUTOINCREMENT,
    full_name VARCHAR(150) NOT NULL,
    phone VARCHAR(20)
);

CREATE TABLE doctor (
    employee_id INTEGER PRIMARY KEY,
    specialty VARCHAR(100) NOT NULL,
    office VARCHAR(20),
    CONSTRAINT fk_doctor_employee FOREIGN KEY (employee_id)
    REFERENCES employee (employee_id)
);

CREATE TABLE registry (
    employee_id INTEGER PRIMARY KEY,
    window_number NUMERIC(3),
    CONSTRAINT fk_registry_employee FOREIGN KEY (employee_id)
    REFERENCES employee (employee_id)
);

CREATE TABLE medical_record (
    record_number INTEGER PRIMARY KEY AUTOINCREMENT,
    creation_date DATETIME DEFAULT CURRENT_TIMESTAMP,
    patient_id INTEGER NOT NULL,
    CONSTRAINT fk_record_patient FOREIGN KEY (patient_id)
    REFERENCES patient (patient_id)
);

CREATE TABLE record_entry (
    entry_id INTEGER PRIMARY KEY AUTOINCREMENT,
    record_number INTEGER NOT NULL,
    entry_number NUMERIC(6),
    entry_date DATETIME DEFAULT CURRENT_TIMESTAMP,
    diagnosis VARCHAR(255),
    complaints VARCHAR(1000),
    CONSTRAINT uq_record_entry UNIQUE (record_number, entry_number),
    CONSTRAINT fk_entry_record FOREIGN KEY (record_number)
    REFERENCES medical_Record (record_number)
);

CREATE TABLE appointment (
    appointment_id INTEGER PRIMARY KEY AUTOINCREMENT,
    appointment_date DATETIME DEFAULT CURRENT_TIMESTAMP,
    appointment_time VARCHAR(10),
    patient_id INTEGER NOT NULL,
    doctor_id INTEGER NOT NULL,
    registry_id INTEGER NOT NULL,
    status VARCHAR(50),
    CONSTRAINT fk_appointment_patient FOREIGN KEY (patient_id)
    REFERENCES patient (patient_id),
    CONSTRAINT fk_appointment_doctor FOREIGN KEY (doctor_id)
    REFERENCES doctor (employee_id),
    CONSTRAINT fk_appointment_registry FOREIGN KEY (registry_id)
    REFERENCES registry (employee_id)
);