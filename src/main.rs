use axum::extract::Query;
use axum::http::Method;
use axum::{routing::get, Json, Router};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::num::IntErrorKind::PosOverflow;
use std::ops::Add;
use tower_http::classify::GrpcFailureClass::Error;
use tower_http::cors::{Any, CorsLayer};

#[derive(Deserialize, Debug)]
struct MathOperation {
    operation: String,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Clone)]
struct Question {
    question: HashMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Clone)]
struct PossibleAnswers {
    x: i32,
    y: i32,
    z: i32,
}
#[derive(Debug, PartialEq, Eq, Serialize, Clone)]
struct QuestionResponse((String, Vec<String>));
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

async fn return_maths_questions(
    operation: Query<MathOperation>,
) -> Result<Json<Vec<QuestionResponse>>, Json<ErrorResponse>> {
    let op = operation.0;
    match op.operation.as_str() {
        "add" => (),
        "subtract" => (),
        "divide" => (),
        "multiply" => (),
        _ => {
            let error_response = ErrorResponse {
                error: format!(
                    "Unrecognised query parameters, please use: add, subtract, divide or multiply"
                ),
            };
            return Err(Json(error_response));
        }
    }
    println!("query param is: {:?}", op);
    let res = generate_custom_questions(op.operation.as_str()).await;
    match res {
        Ok(question) => Ok(Json(question)),
        Err(_) => {
            let error_response = ErrorResponse {
                error: "Failed to generate questions".to_string(),
            };
            Err(Json(error_response))
        }
    }
}

async fn generate_questions() -> Question {
    let mut rng = rand::thread_rng();
    let mut question = Question {
        question: Default::default(),
    };

    while question.question.len() < 10 {
        let x = rng.gen_range(1..20);
        let y = rng.gen_range(1..20);
        let z = rng.gen_range(1..20);


        let question_string = format!("{}+{}", x, y);
        let answer = x + y;
        if !question.question.contains_key(&question_string) {
            // Insert the key-value pair into the HashMap
            question
                .question
                .insert(question_string.clone(), vec![answer.to_string(), x.to_string(), y.to_string(), z.to_string()]);
        } else {
            println!("Key already exists in the HashMap!");
        }
    }

    question
}

async fn generate_custom_questions(mode: &str) -> Result<Vec<QuestionResponse>, axum::Error> {
    let mut question_vec: Vec<QuestionResponse> = Vec::new();
    let mut rng = rand::thread_rng();
    let mut question = Question {
        question: Default::default(),
    };
    let mut answer_vec: Vec<PossibleAnswers> = Vec::new();
    let mut possible_answers = PossibleAnswers {
        x: Default::default(),
        y: Default::default(),
        z: Default::default(),
    };
    while question.question.len() < 10 {
        let x = rng.gen_range(1..20);
        let y = rng.gen_range(1..20);
        let z = rng.gen_range(1..20);
        let (answer, question_string) = match mode {
            "add" => (x + y, format!("{}+{}", x, y)),
            "subtract" => (x - y, format!("{}-{}", x, y)),
            "divide" => (x / y, format!("{}/{}", x, y)),
            "multiply" => (x * y, format!("{}*{}", x, y)),
            _ => {
                return Err(axum::Error::new("Could not generate the questions - we expect either add, subtract, divide or multiply"))
            }
        };
        if !question.question.contains_key(&question_string) {
            // Insert the key-value pair into the HashMap
            possible_answers.x = x;
            possible_answers.y = y;
            possible_answers.z = z;
            question
                .question
                .insert(question_string.clone(), vec![answer.to_string(), x.to_string(), y.to_string(), z.to_string()]);


            answer_vec.push(possible_answers.clone());
            let _ = answer_vec.iter().filter(|num| num.x == answer || num.y == answer || num.z == answer);
        } else {
            println!("Tried to enter a duplicate key.");
        }
    }
    for (key, value) in &question.question {
        question_vec.push(QuestionResponse((key.to_string(), value.to_vec())));
    }
    println!("{:?}", answer_vec);
    println!(" {:?}", question_vec);
    Ok(question_vec)
}
