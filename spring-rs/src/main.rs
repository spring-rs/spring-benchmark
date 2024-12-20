use anyhow::Context;
use spring::{auto_config, App};
use spring_sqlx::sqlx::Row;
use spring_sqlx::ConnectPool;
use spring_sqlx::{sqlx, SqlxPlugin};
use spring_web::get;
use spring_web::{
    axum::response::IntoResponse, error::Result, extractor::Component, WebConfigurator, WebPlugin,
};

#[auto_config(WebConfigurator)]
#[tokio::main(flavor = "multi_thread", worker_threads = 8)] // The number of io-bound threads is set to twice the number of cores
async fn main() {
    App::new()
        .add_plugin(SqlxPlugin)
        .add_plugin(WebPlugin)
        .run()
        .await
}

#[get("/")]
async fn hello_world() -> impl IntoResponse {
    "hello world"
}

#[get("/postgres")]
async fn hello_postgres(Component(db): Component<ConnectPool>) -> Result<String> {
    let version = sqlx::query("select version() as version")
        .fetch_one(&db)
        .await
        .context("sqlx query failed")?
        .get("version");
    Ok(version)
}
