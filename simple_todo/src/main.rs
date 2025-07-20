use grand_line::*;
use serde_json::to_string as json;

// create a sea orm model and graphql object
// id, created_at, updated_at... will be inserted automatically
#[model(no_by_id = true, no_deleted_at = true)]
pub struct Todo {
    pub content: String,
    pub done: bool,
}

// search Todo with filter, sort, pagination
#[search(Todo)]
fn resolver() {
    println!(
        "todoSearch filter={} order_by={} page={}",
        json(&filter)?,
        json(&order_by)?,
        json(&page)?,
    );
    (None, None)
}

// count Todo with filter
#[count(Todo)]
fn resolver() {
    println!("todoCount filter={}", json(&filter)?);
    None
}

// we can also have a custom name
// with extra filter, or default sort in the resolver as well
#[search(Todo)]
fn todoSearch2024() {
    let f = filter_some!(Todo {
        content_starts_with: "2024",
    });
    let o = order_by_some!(Todo [DoneAsc, ContentAsc]);
    (f, o)
}

// get detail of a Todo by id
#[detail(Todo)]
fn resolver() {
    println!("todoDetail id={}", id);
}

// create a new Todo
#[input]
pub struct TodoCreate {
    pub content: String,
}
#[create(Todo)]
fn resolver() {
    println!("todoCreate data={}", json(&data)?);
    let am = active_create!(Todo {
        content: data.content
    });
    am.insert(&tx).await?.into()
}

// update a Todo content
#[input]
pub struct TodoUpdate {
    pub content: String,
}
#[update(Todo)]
fn resolver() {
    println!("todoUpdate id={} data={}", id, json(&data)?);
    let r = Todo::gql_detail(ctx, &tx, &id).await?;
    let am = active_update!(Todo {
        id,
        content: data.content
    });
    am.update(&tx).await?;
    r
}

// custom resolver name and inputs
#[update(Todo, resolver_inputs = true)]
fn todoUpdateDone(id: String, done: bool) {
    println!("todoUpdateDone id={} done={}", id, done);
    let r = Todo::gql_detail(ctx, &tx, &id).await?;
    let am = active_update!(Todo { id, done });
    am.update(&tx).await?;
    r
}

// delete a Todo by id
#[delete(Todo)]
fn resolver() {
    println!("todoDelete id={}", id);
}

// manual query: count number of all done Todo
#[query]
fn todoCountDone() -> u64 {
    let f = filter!(Todo { done: true });
    f.query().count(&tx).await?
}

// manual mutation: delete all done Todo
#[mutation]
fn todoDeleteDone() -> Vec<TodoGql> {
    let f = filter!(Todo { done: true });
    let q = Todo::gql_select_id(ctx, f.query());
    let arr = q.all(&tx).await?;
    let f = filter!(Todo {
        id_in: arr.iter().filter_map(|v| v.id.clone()).collect()
    });
    Todo::delete_many().filter(f.condition()).exec(&tx).await?;
    arr
}

use async_graphql::{EmptySubscription, MergedObject, Schema};
use async_graphql_axum::GraphQL;
use axum::{routing::get_service, serve, Router};
use sea_orm::prelude::*;
use std::error::Error;
use tokio::net::TcpListener;

#[derive(Default, MergedObject)]
struct Query(
    TodoSearchQuery,
    TodoCountQuery,
    TodoSearch2024Query,
    TodoDetailQuery,
    TodoCountDoneQuery,
);

#[derive(Default, MergedObject)]
struct Mutation(
    TodoCreateMutation,
    TodoUpdateMutation,
    TodoUpdateDoneMutation,
    TodoDeleteMutation,
    TodoDeleteDoneMutation,
);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(init_db().await?)
        .finish();

    let app = Router::new().route(
        "/api/graphql",
        get_service(GraphQL::new(schema.clone())).post_service(GraphQL::new(schema)),
    );

    let port = 4000;
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(addr).await?;

    println!("listening on port {}", port);
    serve(listener, app).await?;

    Ok(())
}

async fn init_db() -> Result<DatabaseConnection, Box<dyn Error>> {
    let db = Database::connect("sqlite::memory:").await?;

    db.execute_unprepared(
        "CREATE TABLE todo (
            id TEXT PRIMARY KEY NOT NULL
            , content TEXT NOT NULL
            , done INT(1) NOT NULL
            , created_at TEXT NOT NULL
            , updated_at TEXT
        );",
    )
    .await?;

    Todo::insert_many(vec![
        active_create!(Todo {
            content: "2023 good bye",
            done: true,
        }),
        active_create!(Todo {
            content: "2023 great",
            done: true,
        }),
        active_create!(Todo {
            content: "2024 hello",
            done: false,
        }),
        active_create!(Todo {
            content: "2024 awesome",
            done: false,
        }),
    ])
    .exec(&db)
    .await?;

    Ok(db)
}
