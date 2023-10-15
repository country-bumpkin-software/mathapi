use std::collections::{HashMap};
use axum::{routing::get, Router, Json};
use std::net::SocketAddr;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize)]
struct Question {
    question: HashMap<String, String>,
}
#[tokio::main]
async fn main() {
    let app = Router::new().fallback(
        fallback
    ).route("/", get(handler))
        .route("/random", get(get_random_number))
        .route("/maths_addition", get(return_questions));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8088));

    axum::Server::bind(&addr)
        // Hyper server takes a make service.
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> String {
    "Hello, world!".into()
}
async fn fallback() -> String {
    "Didnt match any route".into()
}
async fn get_random_number() -> String {
    use rand::Rng;
    let num= rand::random::<u16>();

    num.to_string()
}

async fn return_questions() -> Json<Question> {
    let res = generate_questions().await;
    Json(res)
}

async fn generate_questions() -> Question{
    let mut rng = rand::thread_rng();
    let mut question = Question{ question: Default::default() };
    while question.question.len() < 10 {
        let mut x = rng.gen_range(1..20);
        let mut y = rng.gen_range(1..20);
        let questionString = format!("{}+{}", x,y);
        let mut answer = x + y;
        if !question.question.contains_key(&questionString) {
            // Insert the key-value pair into the HashMap
            question.question.insert(questionString.clone(), answer.to_string());
        } else {
            println!("Key already exists in the HashMap!");
        }
        println!("{:?}", question);
    }
    question
}