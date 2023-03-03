# Pathfinder

## How do I run this?

### Requirements to run test

You will need to utilize postman and access the following endpoints
`127.0.0.1:8080/reset_and_pop_db`
and
`127.0.0.1:8080/schedule/FIGTEST`

### Requirements to run api

You will need to download and install postgres and create a `.env` file in the API project like shown in `.env.example`.

## Where's the good stuff?

### Recursion problems

Located in `/api/handlers/schedule.rs`

Catalog is hardcoded in `/api/handlers/catalog.rs`

## dev commands (unused at the moment)

`curl http://127.0.0.1:8080/universities -H "Content-Type: application/json" -d '{"name": "Purdue", "description": "testing"}'`

`curl -X DELETE http://127.0.0.1:8080/universities/1 -H "Content-Type: application/json"`

`curl -X PATCH http://127.0.0.1:8080/universities/2 -H "Content-Type: application/json" -d '{"description": "this description has been updated"}'`

## run frontend

`npm start`

## api

`cargo run`
