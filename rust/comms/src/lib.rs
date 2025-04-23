use std::thread;
use godot::log::{godot_error, godot_print};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use url::Url;
use futures_util::{SinkExt, StreamExt};
use serde::{Serialize, Deserialize};
use serde_json::json;
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};

/// Claims structure for the LiveKit access token.
#[derive(Debug, Serialize, Deserialize)]
struct AccessTokenClaims {
    /// Issuer: your API key.
    iss: String,
    /// Subject: the identity of the user/client.
    sub: String,
    /// Expiration time (as a timestamp).
    exp: usize,
    /// Not before: token valid starting at this time.
    nbf: usize,
    /// The room the client is allowed to join.
    room: String,
}

/// Asynchronous function that encapsulates the LiveKit demo logic.
async fn livekit_demo_async() {
    // Provided LiveKit keys and URL for the test server.
    let livekit_url = "wss://test-z0uyvkcu.livekit.cloud";
    let api_key = "APIgtipu3HnLwQy";
    let api_secret = "uVxrflmZvsjKM52EJHLDLw5ipesmPIIZeEVqeU7VxHlC";
    
    // Define your room name and client identity.
    let room = "demo_room";
    let identity = "rust_client";

    // Set up token validity: valid from now for 1 hour.
    let now = Utc::now();
    let exp = now + Duration::hours(1);

    // Create the JWT claims for the access token.
    let claims = AccessTokenClaims {
        iss: api_key.to_string(),
        sub: identity.to_string(),
        exp: exp.timestamp() as usize,
        nbf: now.timestamp() as usize,
        room: room.to_string(),
    };

    // Encode the token using the API secret.
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(api_secret.as_ref()),
    )
    .expect("Failed to encode token");

    godot_print!("Generated access token: {}", token);

    // Append the token as a query parameter to the LiveKit URL.
    let full_url = format!("{}?access_token={}", livekit_url, token);
    let url = Url::parse(&full_url).expect("Invalid URL");

    // Establish the WebSocket connection.
    let (mut ws_stream, _) = connect_async(url)
        .await
        .expect("Failed to connect to LiveKit server");
    godot_print!("Connected to LiveKit server at {}", livekit_url);

    // Optionally send a JSON join message.
    let join_request = json!({
        "op": "join",
        "room": room,
        "identity": identity,
    });
    ws_stream
        .send(Message::Text(join_request.to_string()))
        .await
        .expect("Failed to send join request");
    godot_print!("Join request sent");

    // Listen for messages from the server.
    while let Some(message) = ws_stream.next().await {
        match message {
            Ok(Message::Text(text)) => {
                godot_print!("Received text message: {}", text);
            }
            Ok(Message::Binary(data)) => {
                godot_print!("Received binary message: {:?}", data);
            }
            Ok(Message::Close(_)) => {
                godot_print!("Server closed the connection");
                break;
            }
            Err(e) => {
                godot_error!("Error receiving message: {}", e);
                break;
            }
            _ => {}
        }
    }
}

/// Synchronous library function that spawns a thread and runs the LiveKit demo in the background.
pub fn run_livekit_demo_background() {
    godot_print!("run_livekit_demo_background");
    // Spawn a new thread and "forget" its handle.
    thread::spawn(|| {
        // Create a new Tokio runtime within the thread.
        let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        rt.block_on(livekit_demo_async());
    });
    // The function returns immediately without waiting for the thread.
}
