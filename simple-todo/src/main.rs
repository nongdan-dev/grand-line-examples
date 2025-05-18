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

// TODO: CODEGEN
#[derive(FromQueryResult)]
struct TodoPartialQueryResult {
    id: Option<String>,
    content: Option<String>,
    done: Option<bool>,
    created_at: Option<DateTimeUtc>,
    updated_at: Option<DateTimeUtc>,
}
impl Into<Todo> for TodoPartialQueryResult {
    fn into(self) -> Todo {
        let mut v = Todo::default();
        if let Some(id) = self.id {
            v.id = id;
        }
        if let Some(content) = self.content {
            v.content = content;
        }
        if let Some(done) = self.done {
            v.done = done;
        }
        if let Some(created_at) = self.created_at {
            v.created_at = created_at;
        }
        v.updated_at = self.updated_at;
        v
    }
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
    let f = todo_filter_some!({
        content_starts_with: "2024",
    });
    let o = todo_order_by_some!([DoneAsc, ContentAsc]);
    (f, o)
}

#[derive(Default)]
pub struct TodoSearch2023Query;
#[async_graphql::Object]
impl TodoSearch2023Query {
    async fn todo_search2023(
        &self,
        ctx: &async_graphql::Context<'_>,
        filter: Option<TodoFilter>,
        order_by: Option<Vec<TodoOrderBy>>,
        page: Option<Pagination>,
    ) -> Result<Vec<Todo>, Box<dyn std::error::Error + Send + Sync>> {
        let tx = ctx.data_unchecked::<DatabaseConnection>().begin().await?;
        let r = {
            // CUSTOM LOGIC FROM RESOVLER
            // declare extra filter and order by
            let (extra_filter, default_order_by) = {
                let f = Some(TodoFilter {
                    content_starts_with: Some("2023".to_string()),
                    ..Default::default()
                });
                let o = Some(vec![(TodoOrderBy::DoneAsc), (TodoOrderBy::ContentAsc)]);
                (f, o)
            };
            // CODEGEN
            // combine filter and extra
            let f = {
                let f1 = filter;
                let f2 = extra_filter;
                if let Some(f1) = f1 {
                    if let Some(f2) = f2 {
                        Some(TodoFilter {
                            and: Some(vec![f1, f2]),
                            ..Default::default()
                        })
                    } else {
                        Some(f1)
                    }
                } else if let Some(f2) = f2 {
                    Some(f2)
                } else {
                    None
                }
            };
            // CODEGEN
            // combine order by and extra
            let o = {
                let o1 = order_by;
                let o2 = default_order_by;
                if let Some(o1) = o1 {
                    if o1.len() > 0 {
                        Some(o1)
                    } else {
                        o2
                    }
                } else {
                    o2
                }
            };
            // CODEGEN
            // todo_db_search
            let mut q = Entity::find();
            if let Some(f) = f {
                q = f.chain(q);
            }
            if let Some(os) = o {
                if os.len() > 0 {
                    for o in os {
                        q = o.chain(q);
                    }
                } else {
                    q = q.order_by_desc(Column::Id);
                }
            } else {
                q = q.order_by_desc(Column::Id);
            }
            let mut offset = 0;
            let mut limit = 100;
            if let Some(p) = page {
                if let Some(o) = p.offset {
                    offset = o;
                }
                if let Some(l) = p.limit {
                    limit = if l > 1000 { 1000 } else { l };
                }
            }
            q = q.offset(offset).limit(limit);
            // CODEGEN
            // partial select
            q = q.select_only();
            let look_ahead = ctx.look_ahead();
            if look_ahead.field("id").exists() {
                q = q.column(TodoColumn::Id)
            }
            if look_ahead.field("content").exists() {
                q = q.column(TodoColumn::Content)
            }
            if look_ahead.field("done").exists() {
                q = q.column(TodoColumn::Done)
            }
            if look_ahead.field("createdAt").exists() {
                q = q.column(TodoColumn::CreatedAt)
            }
            if look_ahead.field("updatedAt").exists() {
                q = q.column(TodoColumn::UpdatedAt)
            }
            let r = q.into_model::<TodoPartialQueryResult>().all(&tx).await?;
            Ok(r.into_iter().map(Into::into).collect())
        };
        // CODEGEN
        // commit
        tx.commit().await?;
        r
    }
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

// manual query: count number of all done Todo
#[query]
fn todoCountDone() -> u64 {
    let f = todo_filter!({
        done: true,
    });
    f.query().count(&tx).await?
}

// manual mutation: delete all done Todo
#[mutation]
fn todoDeleteDone() -> Vec<Todo> {
    let f = todo_filter!({
        done: true,
    });
    let arr = f.query().all(&tx).await?;
    let f = todo_filter!({
        id_in: arr.iter().map(|v| v.id.clone()).collect()
    });
    TodoEntity::delete_many()
        .filter(f.condition())
        .exec(&tx)
        .await?;
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
    TodoSearch2023Query,
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
