#[macro_use]
extern crate rocket;
use rocket::tokio::time::{sleep, Duration};


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
    rocket::build().mount("/", routes![index,world,delay,hello])
}

