use actix_web::{App, HttpResponse, HttpServer, Responder, error, get, post, web};

use actix_web::middleware::Logger;

// use crate::database::operations::establish_connection;
use diesel::r2d2;

pub mod database;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// use database::models::*;
use diesel::prelude::*;

#[get("/patients")]
async fn get_patients(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    // use database::schema::patient::dsl::*;

    let patients = web::block(move || {
        let mut conn = pool.get()?;

        database::operations::fetch_patients(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(match patients {
        Some(p) => HttpResponse::Ok().json(p),
        None => HttpResponse::NotFound().body("No patients found"),
    })

    // let name = req.match_info().get("name").unwrap_or("World");
    // format!("Hello {}!", name)
    // HttpResponse::Ok().json(results)
}

// #[post("/patient")]
// async fn create_patient(conn: &mut PgConnection, new_patient: NewPatient<'_>) -> impl Responder {
//     use database::schema::patient;

//     let new_patient = diesel::insert_into(patient::table)
//         .values(&new_patient)
//         .returning(Patient::as_returning())
//         .get_result(conn)
//         .expect("Error creating a new patient");

//     HttpResponse::Ok().json(new_patient)
// }
// TODO

#[post("/patient")]
async fn create_patient(
    pool: web::Data<DbPool>,
    new_patient: web::Json<database::models::NewPatient>,
) -> actix_web::Result<impl Responder> {
    // Run the blocking Diesel insert inside web::block
    let new_patient = web::block(move || {
        let mut conn = pool.get()?;

        operations::add_new_patient(&mut conn, new_patient.into_inner())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(new_patient))
    // Handle results and return appropriate HTTP responce
    // match new_patient {
    //     Ok(created_patient) => HttpResponse::Created().json(created_patient),
    //     // Ok(Err(e)) => HttpResponse::InternalServerError().body(format!("Database Error: {}", e)),
    //     Err(_) => HttpResponse::InternalServerError().finish(), // Handles web::block thread join errors
    // }
}
//
// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let pool = establish_connection();

    log::info!("Starting HTTP server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            // Add DB pool handle to app data;
            // enables use of `web::Data<DbPool>` extractor
            .app_data(web::Data::new(pool.clone()))
            // Logger middleware emits a log record
            // for every incoming request.
            .wrap(Logger::default())
            .service(echo)
            .service(get_patients)
            .service(create_patient)
        // .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

use dotenvy::dotenv;
use std::env;

use crate::database::operations;

pub fn establish_connection() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to Postgres")
    // PgConnection::establish(&database_url)
    //     .unwrap_or_else(|_| panic!("Error connection to {}", database_url))
}
