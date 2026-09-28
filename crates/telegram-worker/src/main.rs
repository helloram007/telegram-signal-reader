use dotenvy::dotenv;
use grammers_client::{Client, SenderPool};
use std::env;
use grammers_session::storages::SqliteSession;
use std::sync::Arc;
use std::io::{self, Write};
use common::{parse_signal, MessageType, TelegramMessage};
use sqlx::postgres::PgPoolOptions;

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
    
    let api_id: i32 = std::env::var("TG_API_ID")
            .expect("TG_API_ID not found")
            .parse()
            .expect("TG_API_ID must be a number");
    let api_hash = env::var("TG_API_HASH").expect("TG_API_HASH not found");
    let phone = env::var("TG_PHONE").expect("TG_PHONE not found");

    let session = Arc::new(
            SqliteSession::open("telegram.session")
            .await
            .expect("Failed to open Telegram session"),
    );

    //let SenderPool { runner, handle, .. } = SenderPool::new(Arc::clone(&session), api_id);
    let SenderPool {
            runner,
            handle,
            updates,
        } = SenderPool::new(Arc::clone(&session), api_id);
    
    println!("Telegram session opened successfully");

    let client = Client::new(handle);
    let mut updates = client
        .stream_updates(updates, Default::default())
        .await
        .expect("Failed to create Telegram update stream");


    tokio::spawn(runner.run());


    if client.is_authorized().await.expect("Authorization check failed") {
        println!("Client already authorized and ready to use!");
    } else {
        println!("Client is not authorized, you will need to sign in!");

    let token = client
        .request_login_code(&phone, &api_hash)
        .await
        .expect("Failed to request Telegram login code");

    println!("Telegram login code has been sent!");

    print!("Enter the Telegram login code: ");
    io::stdout().flush().unwrap();

    let mut code = String::new();
    io::stdin()
        .read_line(&mut code)
        .expect("Failed to read login code");

    let code = code.trim();

    let _user = client
        .sign_in(&token, code)
        .await
        .expect("Telegram sign-in failed");

    //println!("Successfully signed in as: {:?}", user);
}
let channel_username = "wolvestradingpublic";

let peer = client
    .resolve_username(channel_username)
    .await
    .expect("Failed to resolve username")
    .expect("Channel not found");

let channel = match peer {
    grammers_client::peer::Peer::Channel(channel) => channel,
    _ => panic!("Resolved peer is not a channel"),
};

let peer_ref = channel
    .to_ref()
    .await
    .expect("Failed to convert channel to PeerRef")
    .expect("Channel has no usable PeerRef");

let target_peer_id = peer_ref.id();

println!("Channel resolved successfully!");

let mut messages = client
        .iter_messages(peer_ref)
        .offset_id(60)
        .limit(50);

    while let Some(message) = messages
        .next()
        .await
        .expect("Failed to retrieve messages")
    {   
        let telegram_message = TelegramMessage {
        channel_id: channel
            .id()
            .bot_api_dialog_id()
            .expect("Channel must have a valid Telegram ID"),
        message_id: message.id(),
        date: message.date(),
        text: message.text().to_string(),
        };
        sqlx::query(
            r#"
            INSERT INTO telegram_messages
                (channel_id, message_id, message_date, text)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (channel_id, message_id) DO NOTHING
            "#,
        )
        .bind(telegram_message.channel_id)
        .bind(telegram_message.message_id)
        .bind(telegram_message.date)
        .bind(&telegram_message.text)
        .execute(&pool)
        .await
        .expect("Failed to store Telegram message");


    if let Some(signal) = common::parse_signal(&telegram_message.text) {
        println!("Parsed signal: {:?}", signal);

        let (entry_low, entry_high) = match signal.entry {
            common::Entry::Single(price) => (price, None),
            common::Entry::Range { low, high } => (low, Some(high)),
        };

        sqlx::query(
            r#"
            INSERT INTO trading_signals
                (
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
                )
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT (channel_id, message_id) DO NOTHING
            "#,
        )
        .bind(telegram_message.channel_id)
        .bind(telegram_message.message_id)
        .bind(&signal.symbol)
        .bind(&signal.side)
        .bind(&signal.order_type)
        .bind(entry_low)
        .bind(entry_high)
        .bind(signal.stop_loss)
        .bind(signal.take_profit)
        .bind(signal.tp1)
        .bind(signal.tp2)
        .bind(signal.tp3)
        .execute(&pool)
        .await
        .expect("Failed to store trading signal");
    }
    }

fn process_message(message: &TelegramMessage) -> MessageType {
        let text = message.text.to_lowercase();

        // 1. Structured trading signal
        if text.contains("pair:")
            && text.contains("side:")
            && text.contains("entry:")
        {
            return MessageType::Signal;
        }

        // 2. Promotion / VIP / marketing
        if text.contains("vip")
            || text.contains("join")
            || text.contains("message me")
            || text.contains("free")
        {
            return MessageType::Promotion;
        }

        // 3. Trade management / updates
        if text.contains("active")
            || text.contains("running")
            || text.contains("secure")
            || text.contains("stops")
            || text.contains("stop")
            || text.contains("sl ")
            || text.starts_with("sl")
            || text.contains(" tp")
            || text.starts_with("tp")
            || text.contains("be now")
            || text.contains("break even")
        {
            return MessageType::Update;
        }

        // 4. Analysis / setup discussion
        if text.contains("zone")
            || text.contains("h4")
            || text.contains("h1")
            || text.contains("setup")
            || text.contains("structure")
            || text.contains("resistance")
            || text.contains("support")
            || text.contains("pullback")
            || text.contains("confluence")
        {
            return MessageType::Analysis;
        }

        // 5. Empty text — probably media/image-only message
        if text.trim().is_empty() {
            return MessageType::Media;
        }

        // 6. Everything else for now
        MessageType::Other
    }

    while let Ok(update) = updates.next().await {
        if let grammers_client::update::Update::NewMessage(message) = update {
            if message.peer_id() == target_peer_id {
                if let Some(signal) = common::parse_signal(message.text()) {
                    println!("LIVE SIGNAL DETECTED: {:?}", signal);
                }
            }
        }
    }

}