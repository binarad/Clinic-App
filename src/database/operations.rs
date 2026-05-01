// HERE WILL BE ALL DATABASE MAGIC THINGS
use crate::database::models::{Employee, NewEmployee};
use crate::database::schema::employee;
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

pub async fn delete_employee(target_id: i32) -> Result<usize, String> {
    // Push the heavy database work to a background thread
    tokio::task::spawn_blocking(move || {
        use crate::database::schema::employee::dsl::*;

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

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
