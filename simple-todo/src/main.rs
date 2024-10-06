use grand_line::*;
use serde_json::to_string as json;

// create the graphql input object for pagination
pagination!();

// create a sea orm model and graphql object
// id, created_at, updated_at will be inserted automatically
#[model]
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
    let f = todo_filter!({
        content_starts_with: "2024",
    });
    let o = todo_order_by!([DoneAsc, ContentAsc]);
    (f, o)
}

// get detail of a Todo by id
#[detail(Todo)]
fn resolver() {
    println!("todoDetail id={}", id);
}

// create a new Todo
// must specify `resolver_inputs = true` to use inputs from this resolver
//      use #[macro_model] to automatically generate builtin input data type
#[create(Todo, resolver_inputs = true)]
fn resolver(content: String) {
    println!("todoCreate content={}", content);
    let am = todo_active_create!({ content });
    am.insert(&tx).await?
}

// update a Todo content
// must specify `resolver_inputs = true` to use inputs from this resolver
//      use #[macro_model] to automatically generate builtin input data type
#[update(Todo, resolver_inputs = true)]
fn resolver(id: String, content: String) {
    println!("todoUpdate id={} content={}", id, content);
    let todo = todo_db_detail(&tx, id).await?;
    let am = todo_active_update!({
        content,
        ..todo.into()
    });
    am.update(&tx).await?
}

// toggle a Todo done
// must specify `resolver_inputs = true` to use inputs from this resolver
//      use #[macro_model] to automatically generate builtin input data type
#[update(Todo, resolver_inputs = true)]
fn todoToggleDone(id: String) {
    println!("todoToggleDone id={}", id);
    let todo = todo_db_detail(&tx, id).await?;
    let am = todo_active_update!({
        done: !todo.done,
        ..todo.into()
    });
    am.update(&tx).await?
}

// delete a Todo by id
#[delete(Todo)]
fn resolver() {
    println!("todoDelete id={}", id);
}

// manual query: get total number of Todo
#[query]
fn todoTotal() -> u64 {
    TodoEntity::find().count(&tx).await?
}

// manual mutation: delete all done Todo
#[mutation]
fn todoDeleteDone() -> Vec<Todo> {
    vec![]
}

use async_graphql::{
    http::{AltairConfigOptions, AltairSource, AltairWindowOptions},
    EmptySubscription, MergedObject, Schema,
};
use async_graphql_axum::GraphQL;
use axum::{
    response::{Html, IntoResponse},
    routing::{get, get_service},
    serve, Router,
};
use sea_orm::prelude::*;
use std::error::Error;
use tokio::net::TcpListener;

#[derive(Default, MergedObject)]
struct Query(
    TodoSearchQuery,
    TodoCountQuery,
    TodoSearch2024Query,
    TodoDetailQuery,
    TodoTotalQuery,
);

#[derive(Default, MergedObject)]
struct Mutation(
    TodoCreateMutation,
    TodoUpdateMutation,
    TodoToggleDoneMutation,
    TodoDeleteMutation,
    TodoDeleteDoneMutation,
);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();

    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(init_db().await?)
        .finish();

    let app = Router::new().route("/api/altair", get(altair)).route(
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

async fn altair() -> impl IntoResponse {
    Html(
        AltairSource::build()
            .options(AltairConfigOptions {
                window_options: Some(AltairWindowOptions {
                    endpoint_url: Some("/api/graphql".to_owned()),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .finish(),
    )
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

    TodoEntity::insert_many(vec![
        todo_active_create!({
            content: "2023 good bye",
            done: true,
        }),
        todo_active_create!({
            content: "2023 great",
            done: true,
        }),
        todo_active_create!({
            content: "2024 hello",
            done: false,
        }),
        todo_active_create!({
            content: "2024 awesome",
            done: false,
        }),
    ])
    .exec(&db)
    .await?;

    Ok(db)
}
