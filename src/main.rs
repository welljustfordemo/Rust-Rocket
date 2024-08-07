#[macro_use]
extern crate rocket;

use rocket::tokio::time::{sleep, Duration};
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket_sync_db_pools::{database};
mod schema;
mod models;

#[database("my_db")]
struct DbConn(diesel::PgConnection);

#[get("/users")]
async fn get_users(conn: DbConn) -> Json<Vec<models::User>> {
    conn.run(|c| {
        use schema::users::dsl::*;
        Json(users.load::<models::User>(c).expect("Error loading users"))
    }).await
}

#[post("/users", data = "<new_user>")]
async fn create_user(conn: DbConn, new_user: Json<models::NewUser<'_>>) -> Json<models::User> {
    // 提取 new_user 的数据到新的 String 类型变量中
    let name = new_user.name.to_string();
    let email = new_user.email.to_string();

    conn.run(move |c| {
        use schema::users;
        let new_user_data = models::NewUser {
            name: &name,
            email: &email,
        };
        Json(diesel::insert_into(users::table)
            .values(&new_user_data)
            .get_result(c)
            .expect("Error saving new user"))
    }).await
}
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/world")]
fn world() -> &'static str {
    "world!"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", routes![index, world, delay, hello, get_users, create_user])
}