use crate::graphql::GraphQl;
use actix_web::{post, web, Error, HttpResponse};
use argonautica::Verifier;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct LoginResponse {
    error: bool,
    token: String,
}

impl LoginResponse {
    fn error() -> Self {
        LoginResponse {
            error: true,
            token: String::from(""),
        }
    }

    fn token(token: String) -> Self {
        LoginResponse {
            error: false,
            token: token,
        }
    }
}

#[post("/login")]
pub async fn login(body: web::Bytes, graphql: web::Data<GraphQl>) -> Result<HttpResponse, Error> {
    // Get the json from the query
    let input = input(body);

    // Ask hasura if there is a matching user
    let answer = graphql.user_by_name(&input.name).await;
    let users = &answer["data"]["users"];
    if users.is_empty() {
        // user not found
        return Ok(HttpResponse::Ok().json(LoginResponse::error()));
    }
    let id = users[0]["id"].as_i32().unwrap();
    let hash = users[0]["password"].as_str().unwrap();

    // Now check if the password and the hash match
    let mut verifier = Verifier::default();
    let is_valid = verifier
        .with_hash(hash)
        .with_password(&input.password)
        .with_secret_key(env::var("ARGON2_SECRET").expect("ARGON2_SECRET env var"))
        .verify()
        .unwrap();
    if !is_valid {
        // invalid password
        Ok(HttpResponse::Ok().json(LoginResponse::error()))
    } else {
        // Now generate the token
        let token = generate_token(id);

        // And return !
        let response = LoginResponse::token(token);
        Ok(HttpResponse::Ok().json(response))
    }
}

#[derive(Debug, Serialize)]
struct HasuraClaims {
    #[serde(rename(serialize = "x-hasura-allowed-roles"))]
    x_hasura_allowed_roles: Vec<String>,
    #[serde(rename(serialize = "x-hasura-default-role"))]
    x_hasura_default_role: String,
    #[serde(rename(serialize = "x-hasura-user-id"))]
    x_hasura_user_id: i32,
}

#[derive(Debug, Serialize)]
struct Claims {
    #[serde(rename(serialize = "https://hasura.io/jwt/claims"))]
    claims: HasuraClaims,
}

fn generate_token(userid: i32) -> String {
    let hasura_claims = HasuraClaims {
        x_hasura_allowed_roles: vec![String::from("user")],
        x_hasura_default_role: String::from("user"),
        x_hasura_user_id: userid,
    };
    let claims = Claims {
        claims: hasura_claims,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(
            &env::var("HASURA_GRAPHQL_JWT_SECRET_KEY")
                .expect("HASURA_GRAPHQL_JWT_SECRET_KEY env var not set")
                .as_ref(),
        ),
    )
    .unwrap()
}

#[derive(Debug)]
pub struct JsonInput {
    pub name: String,
    pub password: String,
}

pub fn input(body: web::Bytes) -> JsonInput {
    // Get the json from the query
    let result = json::parse(std::str::from_utf8(&body).unwrap());
    let input: json::JsonValue = result.unwrap();
    JsonInput {
        name : input["input"]["name"].as_str().unwrap().to_string(),
        password : input["input"]["password"].as_str().unwrap().to_string(),
    }
}