# Pathfinder

## Where's the good stuff?

### Recursion problems

Located in `/api/handlers/schedule.rs`

Catalog is hardcoded in `/api/handlers/catalog.rs`

## dev commands

`curl http://127.0.0.1:8080/universities -H "Content-Type: application/json" -d '{"name": "Purdue", "description": "testing"}'`

`curl -X DELETE http://127.0.0.1:8080/universities/1 -H "Content-Type: application/json"`

`curl -X PATCH http://127.0.0.1:8080/universities/2 -H "Content-Type: application/json" -d '{"description": "this description has been updated"}'`

## run frontend

`npm start`

## api

`cargo run`
