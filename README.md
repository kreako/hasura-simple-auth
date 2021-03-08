# hasura-simple-auth

Simple authentication provider for hasura based on the
[doc example in python/flask](https://hasura.io/docs/latest/graphql/core/actions/codegen/python-flask.html#actions-codegen-python-flask) but reimplemented with
[rust/actix](https://actix.rs).

## Hasura expectations

### Models

Expect a `users` table like this :

```sql
CREATE TABLE users (
    id serial NOT NULL,
    name text NOT NULL,
    password text NOT NULL,
    email text NOT NULL,
    PRIMARY KEY (id),
    UNIQUE (name),
    UNIQUE (email)
    );
```

## Actions

In `actions.yaml` :

```yaml
actions:
- name: login
  definition:
    kind: synchronous
    handler: http://127.0.0.1:3000/login
    forward_client_headers: true
  permissions:
  - role: anonymous
- name: signup
  definition:
    kind: synchronous
    handler: http://localhost:3000/signup
    forward_client_headers: true
custom_types:
  enums: []
  input_objects:
  objects:
  - name: LoginToken
  - name: SignupResponse
  scalars: []
```

In `actions.graphql` :

```graphql
type Mutation {
  login (
    name: String!
    password: String!
  ): LoginToken
}   

type LoginToken {
  error : Boolean!
  token : String!
}

type Mutation {
  signup (
    name: String!
    email: String!
    password: String!
  ): SignupResponse
} 

type SignupResponse {
  unknown_error : Boolean
  name_error : Boolean
  email_error : Boolean
  userid : Int
}
```

## Environment variables

In `.env` (for example) :

```
# Secret to hash password
ARGON2_SECRET="very secret argon2"
# Hasura admin Secret
HASURA_GRAPHQL_ADMIN_SECRET="hasura admin secret - keep in sync with hasura deployment"
# JWT key encryption
HASURA_GRAPHQL_JWT_SECRET_KEY="jwt generation symmetric key - keep in sync with hasura deployment"
# Graphql endpoint
HASURA_GRAPHQL_ENDPOINT="http://localhost:8080/v1/graphql"
```