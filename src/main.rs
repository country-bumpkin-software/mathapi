use axum::http::Method;
use axum::{routing::get, Json, Router};
use rand::Rng;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use axum::extract::Query;
use tower_http::cors::{Any, CorsLayer};

#[derive(Deserialize)]
struct MathOperation {
    operation: String,
}
#[derive(Debug, PartialEq, Eq, Serialize)]
struct Question {
    question: HashMap<String, String>,
}
#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET])
        // allow requests from any origin
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .fallback(fallback)
        .route("/", get(handler))
        .route("/maths_addition", get(return_questions))
        .route("/maths_problems", get(return_maths_questions))
        .layer(cors);

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

async fn return_questions() -> Json<Question> {
    let res = generate_questions().await;
    Json(res)
}

async fn return_maths_questions(operation: Query<MathOperation>) -> Json<Question> {
    let op = operation.0;
    let res = generate_custom_questions(op.operation.as_str()).await;
    Json(res)
}

async fn generate_questions() -> Question {
    let mut rng = rand::thread_rng();
    let mut question = Question {
        question: Default::default(),
    };
    while question.question.len() < 10 {
        let x = rng.gen_range(1..20);
        let y = rng.gen_range(1..20);
        let question_string = format!("{}+{}", x, y);
        let answer = x + y;
        if !question.question.contains_key(&question_string) {
            // Insert the key-value pair into the HashMap
            question
                .question
                .insert(question_string.clone(), answer.to_string());
        } else {
            println!("Key already exists in the HashMap!");
        }
        println!("{:?}", question);
    }
    question
}

async fn generate_custom_questions(mode: &str) -> Question {
    let mut rng = rand::thread_rng();
    let mut question = Question {
        question: Default::default(),
    };
    while question.question.len() < 10 {
        let x = rng.gen_range(1..20);
        let y = rng.gen_range(1..20);
        let (answer, question_string) = match mode {
            "add" => (x + y, format!("{}+{}", x, y)),
            "subtract" => (x - y, format!("{}-{}", x, y)),
            "divide" => (x / y, format!("{}/{}", x, y)),
            "multiple" => (x * y, format!("{}*{}", x, y)),
            _ => (0, format!("unrecognised maths operation error")),
        };
        if !question.question.contains_key(&question_string) {
            // Insert the key-value pair into the HashMap
            question
                .question
                .insert(question_string.clone(), answer.to_string());
        } else {
            println!("Key already exists in the HashMap!");
        }
        println!("{:?}", question);
    }
    question
}
