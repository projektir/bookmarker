use chrono::{DateTime, Utc};
use juniper::{Context as JuniperContext, EmptyMutation, RootNode};
use uuid::Uuid;

use crate::database::Pool;

use super::query::Query;
use super::subscription::Subscription;

#[derive(Clone)]
pub struct Context {
    pub db: Pool,
}

impl Context {
    pub fn new(pool: Pool) -> Self {
        Self { db: pool }
    }
}

impl JuniperContext for Context {}

type Schema = RootNode<'static, Query, EmptyMutation<Context>, Subscription>;

pub fn create_schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), Subscription)
}

#[derive(juniper::GraphQLObject)]
#[graphql(description = "A unit")]
pub struct Unit {
    pub name: String,
    pub building: String,
    pub minerals: i32,
    pub gas: i32,
    pub supply: i32,
}

#[derive(juniper::GraphQLObject)]
#[graphql(description = "A bookmark")]
pub struct Bookmark {
    pub id: Uuid,
    pub title: String,
    pub url: String,
    pub icon: String,
    pub notes: String,
    pub tags: Vec<String>,
    pub relevant: DateTime<Utc>,
    pub created: DateTime<Utc>,
}
