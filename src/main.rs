#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use std::pin::Pin;
use std::sync::Arc;

use futures::{Future, FutureExt as _};
use juniper_subscriptions::Coordinator;
use juniper_warp::subscriptions::graphql_subscriptions;
use warp::{http::Response, Filter};

use crate::diesel::Connection;

use graphql::schema::Context;

use database::{establish_connection, ConnectionManager, Pool};

mod database;
mod errors;
mod graphql;
mod models;

embed_migrations!("./migrations");

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let conn =
        diesel::pg::PgConnection::establish("postgres://admin_user:dummy@localhost/bookmarkdb")
            .unwrap();

    embedded_migrations::run_with_output(&conn, &mut std::io::stdout()).unwrap();

    log::debug!("I'm initing a pool!");
    let manager = ConnectionManager::new("postgres://admin_user:dummy@localhost/bookmarkdb");
    Pool::builder().max_size(10).build(manager).unwrap();

    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let log = warp::log("warp_server");

    let homepage = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(format!(
                "<html><h1>juniper_warp</h1><div>visit <a href=\"/graphiql\">/graphiql</a></html>"
            ))
    });

    let db_host = std::env::var("POSTGRES_HOST").unwrap_or("localhost".into());
    let db_url = format!("postgres://admin_user:dummy@{}/bookmarkdb", db_host);

    let pool = match establish_connection(&db_url, 20) {
        Ok(pool) => pool,
        Err(err) => {
            log::error!(
                "Failed to establish db connection for actix web server: {}",
                err
            );
            return Ok(());
        }
    };

    let cloned_pool = Arc::new(pool.clone());
    let context = Context::new((*cloned_pool).clone());
    let schema = graphql::schema::create_schema();

    let coordinator = Arc::new(Coordinator::new(schema));
    let subscription_context = context.clone();
    let subscription_state = warp::any().map(move || subscription_context.clone());

    let subscriptions_handler = warp::path("subscriptions")
        .and(warp::ws())
        .and(subscription_state.clone())
        .and(warp::any().map(move || Arc::clone(&coordinator)))
        .map(
            |ws: warp::ws::Ws,
             context: Context,
             coordinator: Arc<Coordinator<'static, _, _, _, _, _>>| {
                ws.on_upgrade(|websocket| -> Pin<Box<dyn Future<Output = ()> + Send>> {
                    graphql_subscriptions(websocket, coordinator, context)
                        .map(|r| {
                            if let Err(e) = r {
                                println!("Websocket error: {}", e);
                            }
                        })
                        .boxed()
                })
            },
        );

    let warp_context = warp::any().map(move || context.clone());
    let graphql_filter =
        juniper_warp::make_graphql_filter(graphql::schema::create_schema(), warp_context.boxed());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["POST", "GET", "OPTIONS"])
        .allow_headers(vec![
            http::header::ACCEPT,
            http::header::CONTENT_TYPE])
        .max_age(3600);

    warp::serve(
        subscriptions_handler
            .map(|reply| warp::reply::with_header(reply, "Sec-WebSocket-Protocol", "graphql-ws"))
            .or(warp::post().and(warp::path("graphql")).and(graphql_filter))
            .or(warp::get()
                .and(warp::path("playground"))
                .and(juniper_warp::playground_filter(
                    "/graphql",
                    Some("/subscriptions"),
                )))
            .or(homepage)
            .with(cors)
            .with(log),
    )
    .run(([127, 0, 0, 1], 3030))
    .await;

    Ok(())
}
