use core::fmt;
use std::vec;

use axum::{
    Router,
    extract::{
        WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
    routing::get,
};

use futures_util::{SinkExt, StreamExt};
use pcd_viewer_server_02::load_pcd::load_pcd;
use serde::{Deserialize, Serialize};
use tokio::time;
enum LogLevel {
    Info,
    Handler,
    Warning,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            LogLevel::Info => "\x1b[32mINFO\x1b[0m",
            LogLevel::Handler => "\x1b[34mHANDLER\x1b[0m",
            LogLevel::Warning => "\x1b[33mWARNING\x1b[0m",
            LogLevel::Error => "\x1b[31mERROR\x1b[0m",
        };
        write!(f, "{}", text)
    }
}

fn log_message(level: LogLevel, msg: &str) {
    println!("{:12} - {}", format!("--> {}", level), msg);
}

#[derive(Debug, Serialize, Deserialize)]
struct PcdData {
    pcd_name: String,
    process_times: ProcessTimes,
    pcd: Vec<Pcd>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProcessTimes {
    edge: f32,
    send: f32,
    matching: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Pcd {
    data_name: String,
    raw_data: Vec<f64>,
    points_num: u32,
}

async fn handle_ws(ws: WebSocketUpgrade) -> impl IntoResponse {
    // println!("--> {:12} - Accessed /ws", "HANDLER");
    log_message(LogLevel::Handler, "Accessed /ws");

    ws.on_upgrade(handle_socket)
}

async fn handle_socket(socket: WebSocket) {
    log_message(LogLevel::Handler, "Connected to websocket!");

    let (mut sender, mut receiver) = socket.split();

    let mut pcd_data;
    match load_pcd() {
        Ok(data) => {
            pcd_data = data;
            log_message(LogLevel::Info, "Sccuessfully loaded PCD data.");
        }
        Err(e) => {
            log_message(LogLevel::Error, &format!("Failed to load PCD data: {}", e));
            return;
        }
    }

    let mut temp_pcd: Vec<f64> = Vec::new();
    let mut pcd_list: Vec<Vec<f64>> = vec![Vec::new(), Vec::new()];
    let mut counter = 0;
    let mut prev_pcd_rgb: u32 = 0;

    prev_pcd_rgb = pcd_data[0].rgb.clone();
    for pcd in pcd_data {
        if pcd.rgb == prev_pcd_rgb {
            pcd_list[counter].push(pcd.x);
            pcd_list[counter].push(pcd.y);
            pcd_list[counter].push(pcd.z);
        } else {
            counter += 1;
            pcd_list[counter].push(pcd.x);
            pcd_list[counter].push(pcd.y);
            pcd_list[counter].push(pcd.z);
        }

        prev_pcd_rgb = pcd.rgb;
    }
    log_message(LogLevel::Info, format!("Counter: {}", counter).as_str());

    let mut pcd_packet = PcdData {
        pcd_name: "test".to_string(),
        process_times: ProcessTimes {
            edge: 0.05,
            send: 0.07,
            matching: 0.2,
        },
        pcd: vec![
            Pcd {
                data_name: "room".to_string(),
                raw_data: pcd_list[0].clone(),
                points_num: pcd_list[0].len() as u32,
            },
            Pcd {
                data_name: "cr".to_string(),
                raw_data: pcd_list[1].clone(),
                points_num: pcd_list[1].len() as u32,
            },
        ],
    };

    let mut pcd_packet02 = PcdData {
        pcd_name: "test02".to_string(),
        process_times: ProcessTimes {
            edge: 0.0,
            send: 0.0,
            matching: 0.0,
        },
        pcd: vec![],
    };

    let pcd_packet_json = serde_json::to_string(&pcd_packet).unwrap();
    let pcd_packet02_json = serde_json::to_string(&pcd_packet02).unwrap();

    for i in 0..100 {
        if i % 2 == 0 {
            if sender
                .send(Message::Text(pcd_packet_json.clone()))
                .await
                .is_err()
            {
                log_message(LogLevel::Error, "Failed to send PCD data to client.");
                return;
            }
        } else {
            if sender
                .send(Message::Text(pcd_packet02_json.clone()))
                .await
                .is_err()
            {
                log_message(LogLevel::Error, "Failed to send PCD data to client.");
                return;
            }
        }

        time::sleep(time::Duration::from_millis(200)).await;
    }

    log_message(LogLevel::Info, "Closing websocket connection...");

    // loop {}
}

#[tokio::main]
async fn main() {
    log_message(LogLevel::Info, "Starting server...");

    let app = Router::new().route("/ws", get(handle_ws));

    // println!("--> {:12} - started running server on port 3010", "INFO");
    log_message(LogLevel::Info, "started running server on port 3010");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3010").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
