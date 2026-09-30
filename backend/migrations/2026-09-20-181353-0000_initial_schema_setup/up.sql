CREATE TABLE patient (
    patient_id SERIAL PRIMARY KEY,
    full_name VARCHAR(150) NOT NULL,
    birth_date DATE,
    phone VARCHAR(20),
    address VARCHAR(200)
);

CREATE TABLE employee (
    employee_id SERIAL PRIMARY KEY,
    full_name VARCHAR(150) NOT NULL,
    phone VARCHAR(20),
    role TEXT NOT NULL DEFAULT 'Admin',
    email TEXT
);

CREATE TABLE doctor (
    employee_id INTEGER PRIMARY KEY,
    specialty VARCHAR(100) NOT NULL,
    office VARCHAR(20),
    CONSTRAINT fk_doctor_employee FOREIGN KEY (employee_id)
    REFERENCES employee (employee_id) ON DELETE CASCADE
);

CREATE TABLE registry (
    employee_id INTEGER PRIMARY KEY,
    window_number SMALLINT,
    CONSTRAINT fk_registry_employee FOREIGN KEY (employee_id)
    REFERENCES employee (employee_id) ON DELETE CASCADE
);

CREATE TABLE medical_record (
    record_number SERIAL PRIMARY KEY,
    creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    patient_id INTEGER NOT NULL UNIQUE,
    CONSTRAINT fk_record_patient FOREIGN KEY (patient_id)
    REFERENCES patient (patient_id) ON DELETE CASCADE
);

CREATE TABLE record_entry (
    entry_id SERIAL PRIMARY KEY,
    record_number INTEGER NOT NULL,
    entry_number INTEGER,
    entry_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    diagnosis VARCHAR(255),
    complaints VARCHAR(1000),
    CONSTRAINT uq_record_entry UNIQUE (record_number, entry_number),
    CONSTRAINT fk_entry_record FOREIGN KEY (record_number)
    REFERENCES medical_record (record_number) ON DELETE CASCADE
);

CREATE TABLE appointment (
    appointment_id SERIAL PRIMARY KEY,
    appointment_time TIMESTAMP NOT NULL, -- Combined date & time of the actual visit
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- When the booking was entered
    patient_id INTEGER NOT NULL,
    doctor_id INTEGER NOT NULL,
    registry_id INTEGER NOT NULL,
    status VARCHAR(50),
    reason TEXT,
    CONSTRAINT fk_appointment_patient FOREIGN KEY (patient_id)
    REFERENCES patient (patient_id) ON DELETE CASCADE,
    CONSTRAINT fk_appointment_doctor FOREIGN KEY (doctor_id)
    REFERENCES doctor (employee_id) ON DELETE CASCADE,
    CONSTRAINT fk_appointment_registry FOREIGN KEY (registry_id)
    REFERENCES registry (employee_id) ON DELETE CASCADE
);
