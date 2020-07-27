#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, rocket!"
}

use postgres::{Client, NoTls, Error};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use rocket_contrib::json::Json;

#[derive(Serialize)]
#[derive(Deserialize)]
struct Post{
    id: i64,
    title: String,
    body: String,
    create_time: DateTime<Utc>,
    update_time: DateTime<Utc>
}

#[get("/")]
fn list() -> Json<Vec<Post>> {
    let mut client = Client::connect("postgresql://postgres:post123@localhost/blog", NoTls).expect("connect error");
    let mut posts: Vec<Post> = Vec::new();
    for row in client.query("SELECT id, title, body, create_time, update_time FROM post", &[]).expect("query error") {
        let post = Post{
            id: row.get(0),
            title: row.get(1),
            body: row.get(2),
            create_time: row.get(3),
            update_time: row.get(4)
        };
        println!("Post {} is from {}", post.title, post.body);
        posts.push(post)
    }
    Json(posts)
}

#[post("/", data = "<post>")]
fn create(post: Json<Post>) -> Json<Post> {
    let mut client = Client::connect("postgresql://postgres:post123@localhost/blog", NoTls).expect("connect error");
    client.execute(
        "INSERT INTO post (title, body) VALUES ($1, $2)",
        &[&post.title, &post.body],
    ).expect("insert error");
    
    post
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/post", routes![list, create])
        .launch();    
}
