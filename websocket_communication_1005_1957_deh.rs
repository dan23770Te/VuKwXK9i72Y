// websocket_communication.rs
#![feature(proc_macro_hygiene, stmt_expr_attributes)]
#[macro_use]
extern crate rocket;
use rocket::tokio;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::Json;
use rocket::serde::json::Value as JsonValue;
use rocket::serde::json::serde_json::Error as SerdeError;
use rocket::websocket::{self, WebSocket, Message, Result, ClientMessage};
use rocket::serde::json::serde_json::Value;
use tokio::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use serde::{Deserialize, Serialize};

// Define a structure to represent a WebSocket client
struct WebSocketClient {
    socket: WebSocket<'static>,
    client_id: String,
}

// Define a structure to hold the state of connected WebSocket clients
struct ConnectionState {
    clients: Mutex<HashMap<String, WebSocketClient>>,
}

#[derive(Serialize, Deserialize)]
struct WebSocketMessage {
    message: String,
}

#[post("/subscribe", format = "json", data = "<_>")]
fn subscribe<'r>(client_id: String, client: WebSocket<'r>, state: State<ConnectionState>) -> Result<()> {
    let new_client = WebSocketClient {
        socket: client,
        client_id: client_id,
    };
    let mut clients = state.clients.lock().unwrap();
    clients.insert(client_id.clone(), new_client);
    Ok(())
}

#[get("/ws")]
fn ws_endpoint() -> rocket::handler::Handler<WebSocket> {
    websocket::on_message()
        .map(|message| match message {
            ClientMessage::Text(text) => {
                let state = ConnectionState {
                    clients: Mutex::new(HashMap::new()),
                };
                let mut clients = state.clients.lock().unwrap();
                for (_id, client) in clients.iter_mut() {
                    client.socket.send_message(&text);
                }
                Ok(())
            },
            _ => Err(rocket::Error::internal("This should be a text message").into()),
        }).boxed()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(ConnectionState {
            clients: Mutex::new(HashMap::new()),
        })
        .mount("/ws", routes![subscribe, ws_endpoint])
}
