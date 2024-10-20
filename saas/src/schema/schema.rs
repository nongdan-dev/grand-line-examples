use crate::prelude::*;
use async_graphql::*;
use grand_line::*;

#[derive(Default, MergedObject)]
pub struct Query(
    // todo
    TodoSearchQuery,
    TodoCountQuery,
    TodoSearch2024Query,
    TodoDetailQuery,
    // user
    UserSearchQuery,
);

#[derive(Default, MergedObject)]
pub struct Mutation(
    // todo
    TodoCreateMutation,
    TodoUpdateMutation,
    TodoDeleteMutation,
    // user
);

pub fn build_schema() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
}
