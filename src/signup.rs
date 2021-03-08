use crate::graphql::GraphQl;
use actix_web::{post, web, Error, HttpResponse};
use argonautica::Hasher;
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct SignupResponse {
    unknown_error: bool,
    name_error: bool,
    email_error: bool,
    userid: Option<i32>,
}

impl SignupResponse {
    fn unknown_error() -> Self {
        SignupResponse {
            unknown_error: true,
            name_error: false,
            email_error: false,
            userid: None,
        }
    }

    fn name_error() -> Self {
        SignupResponse {
            unknown_error: false,
            name_error: true,
            email_error: false,
            userid: None,
        }
    }

    fn email_error() -> Self {
        SignupResponse {
            unknown_error: false,
            name_error: false,
            email_error: true,
            userid: None,
        }
    }

    fn userid(userid: i32) -> Self {
        SignupResponse {
            unknown_error: false,
            name_error: false,
            email_error: false,
            userid: Some(userid),
        }
    }
}

#[post("/signup")]
pub async fn signup(body: web::Bytes, graphql: web::Data<GraphQl>) -> Result<HttpResponse, Error> {
    // Get the json from the query
    let input = input(body);

    // hash the password
    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(&input.password)
        .with_secret_key(env::var("ARGON2_SECRET").expect("ARGON2_SECRET env var"))
        .hash()
        .unwrap();

    // Ask hasura to create the user
    let res = graphql.insert_user(&input.name, &input.email, &hash).await;
    if res.has_key("data") {
        let userid = res["data"]["insert_users"]["returning"][0]["id"].as_i32().unwrap();
        Ok(HttpResponse::Ok().json(SignupResponse::userid(userid)))
    } else {
        let message = res["errors"][0]["message"].as_str().unwrap();
        if message.contains("users_name") {
            Ok(HttpResponse::Ok().json(SignupResponse::name_error()))
        } else if message.contains("users_email") {
            Ok(HttpResponse::Ok().json(SignupResponse::email_error()))
        } else {
            Ok(HttpResponse::Ok().json(SignupResponse::unknown_error()))
        }
    }
}


#[derive(Debug)]
pub struct JsonInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub fn input(body: web::Bytes) -> JsonInput {
    // Get the json from the query
    let result = json::parse(std::str::from_utf8(&body).unwrap());
    let input: json::JsonValue = result.unwrap();
    JsonInput {
        name : input["input"]["name"].as_str().unwrap().to_string(),
        email : input["input"]["email"].as_str().unwrap().to_string(),
        password : input["input"]["password"].as_str().unwrap().to_string(),
    }
}