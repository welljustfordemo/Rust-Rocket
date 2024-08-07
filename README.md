# How to run in debug model
Need to install `cargo-watch` first
Steps:
1. `cargo install cargo-watch`
2. `cargo watch -x run`

# How to run test
`cargo test`

# How to Integrate Diesel Database
**NOTICE:** Make sure you setup the database first , in my case , I use docker-compose to setup the database
```shell
1. Set up these in `Cargo.toml`
```toml
[dependencies]
rocket = { version = "0.5.1",features = ["json"] }
diesel = { version = "2.2.2", features = ["postgres","r2d2"] }
dotenvy = "0.15.0"
serde = { version = "1.0.152", features = ["derive"] }
serde_json = "1.0"

tokio = { version = "1", features = ["full"] }
rocket_sync_db_pools = { version = "0.1.0-rc.1", features = ["diesel_postgres_pool"] }
```
2. Set up Diesel CLI
`cargo install diesel_cli --no-default-features --features postgres`

3. Configure PostgreSQL:
   Create a .env file in your project root:
```.dotenv
DATABASE_URL=postgres://user:user_password@localhost/my_database
```
4. Initialize Diesel
```shell
diesel setup
```

5. Create a file called `Rocket.toml` if not have it yet, and add the following:
```toml
[default.databases]
my_db = { url = "postgres://user:user_password@localhost/my_database" }
```

6. Set up your database schema
```shell
diesel migration generate create_users
```

7. define the schema in the migration file in `up.sql`
```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE
);
```
`down.sql`
```sql
DROP TABLE users;
```

8. Run the migration
```shell
diesel migration run
```

9. Create `models.rs` in /src
```rust
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
}
```

10. Finally config as below in `main.rs`
```rust
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

#[launch]
fn rocket() -> _ {
   rocket::build()
           .attach(DbConn::fairing())
           .mount("/", routes![get_users, create_user])
}
```

11. Run the project
```shell
cargo watch -x run
```