mod schema;

use std::io::Result;
use std::time::Duration;

use actix_web::web::{Data, Json};
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{Insertable, Queryable};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use time::format_description::well_known::Iso8601;
use time::OffsetDateTime;
use uuid::Uuid;

#[serde_as]
#[derive(Deserialize)]
struct Heartbeat {
  source: String,
  #[serde_as(as = "DisplayFromStr")]
  duration: humantime::Duration,
}

#[serde_as]
#[derive(Insertable, Queryable, Serialize)]
#[diesel(table_name = schema::heartbeat)]
struct HeartbeatEntity {
  id: Uuid,
  source: String,
  #[serde_as(as = "Iso8601")]
  expiry: OffsetDateTime,
}

#[post("/")]
async fn insert(pool: Data<Pool<AsyncPgConnection>>, request: Json<Heartbeat>) -> impl Responder {
  let Json(Heartbeat { source, duration }) = request;
  let duration: Duration = duration.into();

  diesel::insert_into(schema::heartbeat::table)
    .values(HeartbeatEntity {
      id: Uuid::new_v4(),
      source,
      expiry: OffsetDateTime::now_utc() + duration,
    })
    .execute((&mut pool.get().await.unwrap()).as_mut())
    .await
    .unwrap();

  HttpResponse::Ok()
    .content_type("application/json")
    .body("{}")
}

#[get("/")]
async fn list(pool: Data<Pool<AsyncPgConnection>>) -> impl Responder {
  use schema::heartbeat::dsl::*;
  let result = schema::heartbeat::table
    .filter(expiry.gt(OffsetDateTime::now_utc()))
    .load::<HeartbeatEntity>((&mut pool.get().await.unwrap()).as_mut())
    .await
    .unwrap();

  HttpResponse::Ok().json(result)
}

#[actix_web::main]
async fn main() -> Result<()> {
  let server = HttpServer::new(|| {
    let connection_manager =
      AsyncDieselConnectionManager::<AsyncPgConnection>::new("postgres://postgres:p@localhost");
    let pool: Pool<AsyncPgConnection> = Pool::builder(connection_manager).build().unwrap();

    App::new()
      .app_data(Data::new(pool))
      .service(insert)
      .service(list)
  });
  server.bind(("0.0.0.0", 8080))?.run().await
}
