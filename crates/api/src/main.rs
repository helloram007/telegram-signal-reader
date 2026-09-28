use axum::{routing::get, Router};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use serde::{Serialize, Deserialize};


#[derive(serde::Serialize, sqlx::FromRow)]
struct MessageResponse {
    id: i64,
    channel_id: i64,
    message_id: i32,
    message_date: chrono::DateTime<chrono::Utc>,
    text: String,
}

#[derive(serde::Serialize, sqlx::FromRow)]
struct SignalResponse {
    id: i64,
    channel_id: i64,
    message_id: i32,
    symbol: String,
    side: String,
    order_type: String,
    entry_low: f64,
    entry_high: Option<f64>,
    stop_loss: f64,
    take_profit: Option<f64>,
    tp1: Option<f64>,
    tp2: Option<f64>,
    tp3: Option<f64>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL not found");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    println!("PostgreSQL connection established!");

    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/messages", get(messages))
        .route("/api/signals", get(signals))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind server");

    println!("API server listening on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("API server failed");

}

async fn health(
    axum::extract::State(pool): axum::extract::State<sqlx::PgPool>,
) -> &'static str {
        sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .expect("Database health check failed");

    "ok"
}

async fn messages(
    axum::extract::State(pool): axum::extract::State<sqlx::PgPool>,
) -> axum::Json<Vec<MessageResponse>> {
    let rows = sqlx::query_as::<_, MessageResponse>(
        r#"
        SELECT
            id,
            channel_id,
            message_id,
            message_date,
            text
        FROM telegram_messages
        ORDER BY message_id DESC
        LIMIT 20
        "#,
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch messages");

    axum::Json(rows)
}

async fn signals(
    axum::extract::State(pool): axum::extract::State<sqlx::PgPool>,
) -> axum::Json<Vec<SignalResponse>> {
    let rows = sqlx::query_as::<_, SignalResponse>(
        r#"
        SELECT
            id,
            channel_id,
            message_id,
            symbol,
            side,
            order_type,
            entry_low,
            entry_high,
            stop_loss,
            take_profit,
            tp1,
            tp2,
            tp3
        FROM trading_signals
        ORDER BY message_id DESC
        LIMIT 20
        "#,
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch trading signals");

    axum::Json(rows)
}