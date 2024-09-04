use anyhow::Context;
use spring::{auto_config, get, App};
use spring_sqlx::sqlx::Row;
use spring_sqlx::ConnectPool;
use spring_sqlx::{sqlx, SqlxPlugin};
use spring_web::{
    axum::response::IntoResponse, error::Result, extractor::Component, WebConfigurator, WebPlugin,
};

#[auto_config(WebConfigurator)]
#[tokio::main]
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

#[get("/mysql")]
async fn hello_mysql(Component(db): Component<ConnectPool>) -> Result<String> {
    let version = sqlx::query("select version() as version")
        .fetch_one(&db)
        .await
        .context("sqlx query failed")?
        .get("version");
    Ok(version)
}
