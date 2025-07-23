use crate::prelude::*;
use async_graphql::{extensions::Tracing, EmptyMutation, EmptySubscription, MergedObject, Schema};
use std::sync::Arc;

#[derive(Default, MergedObject)]
pub struct Query(
    TodoSearchQuery,
    TodoCountQuery,
    UserSearchQuery,
    UserCountQuery,
);

pub fn init_schema(db: Arc<DatabaseConnection>) -> Schema<Query, EmptyMutation, EmptySubscription> {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        // TODO: add feature flag tracing
        .extension(Tracing)
        .extension(GrandLineExtension)
        .data(db)
        .finish()
}
