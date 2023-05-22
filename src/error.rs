use actix_web::ResponseError;
use std::fmt::{Debug, Display};

#[derive(Debug, Display)]
pub enum ApplicationError {
  Deadpool(diesel_async::pooled_connection::deadpool::PoolError),
  Diesel(diesel::result::Error),
}

impl ResponseError for ApplicationError {}
