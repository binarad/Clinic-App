use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::database::models::{Employee, NewEmployee};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn show_employees() -> Vec<Employee> {
    use crate::database::schema::employee::dsl::*;

    let connection = &mut establish_connection();

    // let results = employee
    //     .limit(5)
    //     .select(Employee::as_select())
    //     .load(connection)
    //     .expect("Error loading employees");
    // println!("Displaying employees");
    // for employeee in results {
    //     println!("ID: {}", employeee.employee_id.unwrap());
    //     println!("-------------------\n");
    //     println!("Full Name: {}", employeee.full_name);
    //     println!("Phone: {}", employeee.phone.unwrap());
    // }

    employee
        .limit(5)
        .select(Employee::as_select())
        .load(connection)
        .expect("Error loading employee")
}

pub fn add_employee(
    conn: &mut SqliteConnection,
    full_name: &str,
    phone: Option<&str>,
    role: &str,
    email: Option<&str>,
) -> Employee {
    use crate::database::schema::employee;
    let new_employee = NewEmployee {
        full_name,
        phone,
        role,
        email,
    };

    diesel::insert_into(employee::table)
        .values(&new_employee)
        .returning(Employee::as_returning())
        .get_result(conn)
        .expect("Error saving new employee")
}
