// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::{Read, Write};
use std::time::Duration;
use std::net::TcpStream;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::mem::drop;
use std::net::Shutdown;

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

//glob variables (TODO: this is not really the safe way))
static mut CONNECTION: bool = false;
static mut ALREADY_SENT_MESSAGE: bool = false;
lazy_static! {
  static ref TCP_STREAM: Mutex<Option<TcpStream>> = Mutex::new(None);
}

fn send_data(input: String) -> String{
  let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), DEV_PORT);
  let result = TcpStream::connect_timeout(&socket, Duration::new(CONNECTION_TIME, 0));
  let mut tcp_stream = match result {
      Ok(tcp_stream) => {
        //drone connection approved
        unsafe {CONNECTION = true;}

        tcp_stream
      },
      Err(e) => {
        //drone connection refused
        unsafe {CONNECTION = false;}
        println!("connection refused: {}", e);
        return "connection refused".to_string();
      },
  };

  tcp_stream.write_all(input.as_bytes()).unwrap();
  tcp_stream.flush().unwrap();

  let mut buffer = [0, 255]; //TODO: možná tady bude problém že se nám přečte jen 255 bytů
  let size = tcp_stream.read(&mut buffer).unwrap();
  let message = String::from_utf8_lossy(&buffer[..size]);

  tcp_stream.shutdown(Shutdown::Both).expect("shutdown call failed");
  drop(tcp_stream);
  return message.to_string();
}

//message receivers

//settings
#[tauri::command]
fn connect_to_drone() {
  println!("I was invoked from JS!");
  unsafe {
    ALREADY_SENT_MESSAGE = false;

    let server_response = send_data("hello".to_string());
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
fn send_test_data() -> String{
  let server_response = send_data("ledon".to_string());
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
fn move_drone(side: String) {
  println!("move {}", side);
  send_data("move-".to_string() + &side);
}

#[tauri::command]
fn rotate_drone(rot_side: i32) {
  println!("rotate {}", rot_side);
  send_data("move-".to_string() + &rot_side.to_string());
}

#[tauri::command]
fn take_off() {
  println!("take_off");
  send_data("take_off".to_string());
}

#[tauri::command]
fn land() {
  println!("land");
  send_data("land".to_string());
}

#[tauri::command]
fn move_joystick(coords: [i32; 2]) {
  println!("move_joystick: {} {}", coords[0], coords[1]);
  send_data("joy-".to_string() + &coords[0].to_string() + &coords[1].to_string());
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
  tauri::Builder::default()
  //messsage handlers
  .invoke_handler(tauri::generate_handler![connect_to_drone, move_drone, rotate_drone, take_off, land, move_joystick, check_conn, send_connection_data, send_test_data])
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}
