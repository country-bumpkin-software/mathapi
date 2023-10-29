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
    question: String,
    answer: String,
    choices: PossibleAnswers,
}

#[derive(Debug, PartialEq, Eq, Serialize, Clone)]
struct Questions {
    question_vec: Vec<Question>,
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

// async fn return_questions() -> Json<Questions> {
//     let res = generate_questions().await;
//     Json(res)
// }

async fn return_maths_questions(
    operation: Query<MathOperation>,
) -> Result<Json<Questions>, Json<ErrorResponse>> {
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
        Ok(question) => Ok(question),
        Err(_) => {
            let error_response = ErrorResponse {
                error: "Failed to generate questions".to_string(),
            };
            Err(Json(error_response))
        }
    }
}

async fn generate_custom_questions(mode: &str) -> Result<Json<Questions>, axum::Error> {
    let mut rng = rand::thread_rng();
    let mut question = Question {
        question: Default::default(),
        answer: Default::default(),
        choices: PossibleAnswers{
            x: 0,
            y: 0,
            z: 0,
        } ,
    };
    let mut questions = Questions {
        question_vec: vec![],
    };
    let mut pos_answers = PossibleAnswers { x: 0, y: 0, z: 0 };

    while questions.question_vec.len() < 10 {
        let x = rng.gen_range(1..20);
        let y = rng.gen_range(1..20);

        let question_string = format!("{}+{}", x, y);
        let answer = x + y;

        question.question = question_string.to_string();
        question.answer = answer.to_string();

            pos_answers.x = rng.gen_range(-20..answer) + rng.gen_range(answer..answer + 5);
            pos_answers.y = rng.gen_range(-10..answer-5) + rng.gen_range(answer..answer + 7);
            pos_answers.z = rng.gen_range(-5..answer-7) + rng.gen_range(answer..answer + 9);


        question.choices = pos_answers.clone();

        questions.question_vec.push(question.clone());
    }

    Ok(Json(questions))
}
