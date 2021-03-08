use actix_web::client::Client;
use json::JsonValue;
use std::env;

pub struct GraphQl {
    pub endpoint: String,
}

impl GraphQl {
    pub fn new() -> Self {
        GraphQl {
            endpoint: env::var("HASURA_GRAPHQL_ENDPOINT")
                .expect("HASURA_GRAPHQL_ENDPOINT variable not set"),
        }
    }

    pub async fn run_query(&self, query: &'static str, variables: JsonValue) -> JsonValue {
        let payload = json::object! {
            "query": query,
            "variables": variables,
        };
        let client = Client::default();
        let mut res = client
            .post(&self.endpoint)
            .send_body(payload.dump())
            .await
            .unwrap();
        let body = res.body().await.unwrap();
        let result = json::parse(std::str::from_utf8(&body).unwrap());
        result.unwrap()
    }

    pub async fn user_by_name(&self, name: &str) -> JsonValue {
        let query = "query UserByName($name: String!) {
            users(where: {name: {_eq: $name}}, limit: 1) {
                id
                name
                password
            }
        }";
        self.run_query(query, json::object! {"name": name}).await
    }

    pub async fn insert_user(&self, name: &str, email: &str, hash: &str) -> JsonValue {
        let query = "mutation InsertUser($name: String!, $email: String!, $password: String!) {
            insert_users(objects: {name: $name, email: $email, password: $password}) {
                returning {
                    id
                }
            }
        }";
        self.run_query(query, json::object! {"name": name, "email": email, "password": hash}).await
    }
}
