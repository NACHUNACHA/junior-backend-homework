#[macro_use]
extern crate rocket;

mod models;
mod controllers;
mod repositories;
mod services;
mod s3_client;
mod dtos;

use std::env;
// use rocket::fs::FileServer;
use rocket::{Build, Rocket};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;

async fn get_database_pool() -> MySqlPool {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    MySqlPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to MySQL")
}

#[launch]
async fn rocket() -> Rocket<Build> {
    dotenv::dotenv().ok();

    let pool = get_database_pool().await;

    rocket::build()
        .manage(pool)
        .mount("/", 
               routes![
                   controllers::file_controller::list_files,
                   controllers::file_controller::upload_file]
        )
}