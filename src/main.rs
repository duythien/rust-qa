// import the Filter trait from warp
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use warp::{
    filters::cors::CorsForbidden, http::Method, http::StatusCode, reject::Reject, Filter,
    Rejection, Reply,
};

use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize, Eq, Hash, PartialEq, Clone)]
struct QuestionId(String);

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}
#[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId {}
async fn get_questions() -> Result<impl Reply, Rejection> {
    // Fetch the data (this is just a placeholder; provide your logic here)
    // let questions = vec![
    //     "What is Rust?",
    //     "How does ownership work?",
    //     "What are traits?",
    // ];
    // // Convert the data to JSON
    // let json = warp::reply::json(&questions);

    Ok(Store::init())
}
async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(InvalidId) = r.find() {
        Ok(warp::reply::with_status(
            "No valid ID presented".to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
#[derive(Clone)]
struct Store {
    questions: HashMap<QuestionId, Question>,
}
impl Store {
    fn new() -> Self {
        Store {
            questions: Self::init(),
        }
    }
    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../question.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }
}
#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions)
        .recover(return_error);
    let routes = get_items.with(cors);
    // start the server and pass the route filter to it
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
