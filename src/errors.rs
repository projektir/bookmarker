use juniper::graphql_value;
use log::error;
use thiserror::Error;

use crate::database::DatabaseError;

#[derive(Error, Debug)]
pub enum BookmarkerError {
    #[error("DatabaseError: {0}")]
    DatabaseError(#[from] DatabaseError),
}

impl juniper::IntoFieldError for BookmarkerError {
    fn into_field_error(self) -> juniper::FieldError {
        error!("{}", self);
        juniper::FieldError::new(
            "Internal Error",
            graphql_value!({
                "type": "Internal Error"
            }),
        )
    }
}
