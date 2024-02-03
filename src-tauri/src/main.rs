// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::{Read, Write};
use std::time::Duration;
use std::net::TcpStream;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Mutex, Arc};
use tauri::Manager;
use serde_json::Value;

//info
//drone addr: 192.168.4.1
//drone port: 33333

//localhost info
//localhost addr: 127.0.0.1
//localhost port: 12345

//variables
const PORT: u16 = 33333;
const DEV_PORT: u16 = 12345;
const CONNECTION_TIME: u64 = 5;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

//glob variables (TODO: this127 is not really the safe way))
static mut CONNECTION: bool = false;
static mut ALREADY_SENT_MESSAGE: bool = false;

//other functions
fn send_data(tcp_stream: Arc<Mutex<TcpStream>>, input: String) -> String{

  let mut stream = tcp_stream.lock().unwrap();

  stream.write_all(input.as_bytes()).unwrap();
  stream.flush().unwrap();

  let mut buffer = [0, 255]; //TODO: možná tady bude problém že se nám přečte jen 255 bytů
  let size = stream.read(&mut buffer).unwrap();
  let message = String::from_utf8_lossy(&buffer[..size]);

  return message.to_string();
}

//message receivers

fn check_conn() -> String {
  unsafe {
    println!("{}", ALREADY_SENT_MESSAGE);
    if ALREADY_SENT_MESSAGE == false{
      ALREADY_SENT_MESSAGE = true;
      match CONNECTION{
        true => "connected".into(),
        false => "not_connected".into(),
      }
    }
    else {
      return "blank".into(); //sending blank message
    }
  }
}

//project

fn main() {
  //initial connection
  //try initial connection

  tauri::Builder::default()
  .setup(|app| {

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), DEV_PORT);
    let tcp_stream = Arc::new(Mutex::new(TcpStream::connect_timeout(&socket, Duration::new(CONNECTION_TIME, 0)).expect("Server is offline?")));

    // listen to the `event-name` (emitted on any window)
    let id_drone_command = app.listen_global("drone-command", move |event| {
      let data = event.payload().unwrap();
      let deserialized: Value = serde_json::from_str(&data).unwrap();
      
      let command_type = deserialized["command"].as_str().unwrap();
      let command_value: String = deserialized["value"].to_string();
      println!("{}", command_type.to_string());

      match command_type{
        "connect-to" | "rotate" | "move-joystick" => {
          send_data(tcp_stream.clone(), command_value);
        },
        "take-off" | "land" => {
          send_data(tcp_stream.clone(), command_type.to_string());
        },
        _ => println!("tvoje matinka")
      }
    });

    // emit the `event-name` event to all webview windows on the frontend
    app.emit_all("event-name", Payload { message: "Tauri is awesome!".into() }).unwrap();
    Ok(())
  })
  //messsage handlers
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}