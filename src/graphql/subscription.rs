use std::pin::Pin;
use std::time::Duration;

use futures::Stream;
use juniper::FieldError;

use super::schema::{Context, Unit};

type UnitsStream = Pin<Box<dyn Stream<Item = Result<Unit, FieldError>> + Send>>;

pub struct Subscription;

#[juniper::graphql_subscription(Context = Context)]
impl Subscription {
    async fn units() -> UnitsStream {
        let mut counter = 0;
        let stream = tokio::time::interval(Duration::from_secs(5)).map(move |_| {
            counter += 1;

            Ok(Unit {
                name: "mm".to_string(),
                building: "mm".to_string(),
                minerals: counter,
                gas: 5,
                supply: 6,
            })
        });

        Box::pin(stream)
    }
}
