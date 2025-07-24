use crate::prelude::*;
use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema, extensions::Tracing};

#[derive(Default, MergedObject)]
pub struct Query(
    TodoSearchQuery,
    TodoCountQuery,
    UserSearchQuery,
    UserCountQuery,
    OrgSearchQuery,
    OrgCountQuery,
);

pub fn init_schema(db: Arc<DatabaseConnection>) -> Schema<Query, EmptyMutation, EmptySubscription> {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        // TODO: add feature flag tracing
        .extension(Tracing)
        .extension(GrandLineExtension)
        .data(db)
        .finish()
}
