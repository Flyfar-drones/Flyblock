// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::borrow::Borrow;
use std::io::{Read, Write};
use std::process;
use std::time::Duration;
use std::net::TcpStream;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use lazy_static::lazy_static;
use tauri::State;
use std::sync::Mutex;
use std::mem::drop;
use std::net::Shutdown;
use tauri::Manager;

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

fn send_data(tcp_stream: State<TcpStream>, input: String) -> String{
  let mut stream = tcp_stream.inner();

  stream.write_all(input.as_bytes()).unwrap();
  stream.flush().unwrap();

  let mut buffer = [0, 255]; //TODO: možná tady bude problém že se nám přečte jen 255 bytů
  let size = stream.read(&mut buffer).unwrap();
  let message = String::from_utf8_lossy(&buffer[..size]);

  tcp_stream.shutdown(Shutdown::Both).expect("shutdown call failed");
  drop(tcp_stream);
  return message.to_string();
}

//message receivers

//settings
#[tauri::command]
fn connect_to_drone(stream: tauri::State<TcpStream>) {
  let tcp_stream = stream.clone();

  println!("I was invoked from JS!");
  unsafe {
    ALREADY_SENT_MESSAGE = false;

    let server_response = send_data(tcp_stream, "hello".to_string());
    println!("Server says: {}", server_response);
  }
}

#[tauri::command]
fn send_connection_data() -> String{
  unsafe {
    if CONNECTION{
      "connected".into()
    }
    else{
      "not_connected".into()
    }
  }
}

#[tauri::command]
fn send_test_data(stream: tauri::State<TcpStream>) -> String{
  let tcp_stream = stream.clone();
  let server_response = send_data(tcp_stream, "ledon".to_string());
  let response = server_response.as_str();

  println!("Server says: {}", server_response);

  match response{
    "ok" => "ok".to_string().into(),
    "not_ok" => "not_ok".to_string().into(),
    &_ => return "random_error".to_string()
  }
}

//control
#[tauri::command]
fn move_drone(stream: tauri::State<TcpStream>, side: String) {
  let tcp_stream = stream.clone();

  println!("move {}", side);
  send_data(tcp_stream, "move-".to_string() + &side);
}

#[tauri::command]
fn rotate_drone(stream: tauri::State<TcpStream>, rot_side: i32) {
  let tcp_stream = stream.clone();

  println!("rotate {}", rot_side);
  send_data(tcp_stream, "move-".to_string() + &rot_side.to_string());
}

#[tauri::command]
fn take_off(stream: tauri::State<TcpStream>) {
  let tcp_stream = stream.clone();

  println!("take_off");
  send_data(tcp_stream, "take_off".to_string());
}

#[tauri::command]
fn land(stream: tauri::State<TcpStream>) {
  let tcp_stream = stream.clone();

  println!("land");
  send_data(tcp_stream, "land".to_string());
}

#[tauri::command]
fn move_joystick(stream: tauri::State<TcpStream>, coords: [i32; 2]) {
  let tcp_stream = stream.clone();

  println!("move_joystick: {} {}", coords[0], coords[1]);
  send_data(tcp_stream, "joy-".to_string() + &coords[0].to_string() + &coords[1].to_string());
}

#[tauri::command]
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
  let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), DEV_PORT);
  let stream = TcpStream::connect_timeout(&socket, Duration::new(CONNECTION_TIME, 0)).expect("Server is offline?");

  tauri::Builder::default()
  .setup(|app| {
    // listen to the `event-name` (emitted on any window)
    let id_move = app.listen_global("test", |event| {
      let data = event.payload().unwrap();
      println!("{}", data);
    });

    // emit the `event-name` event to all webview windows on the frontend
    app.emit_all("event-name", Payload { message: "Tauri is awesome!".into() }).unwrap();
    Ok(())
  })
  //messsage handlers
  .invoke_handler(tauri::generate_handler![connect_to_drone, move_drone, rotate_drone, take_off, land, move_joystick, check_conn, send_connection_data, send_test_data])
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}