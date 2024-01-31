// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::{Read, Write};
use std::time::Duration;
use std::net::TcpStream;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
//use tokio::net::TcpStream;

//variables
const PORT: u16 = 33333;
const CONNECTION_TIME: u64 = 5;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

//glob variables (TODO: this is not really the safe way))
static mut CONNECTION: bool = false;
static mut ALREADY_SENT_MESSAGE: bool = false;

//message receivers
 
//settings
#[tauri::command]
async fn connect_to_drone() {
  println!("I was invoked from JS!");
  unsafe {ALREADY_SENT_MESSAGE = false}

  let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 4, 1)), PORT);
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

        return println!("connection refused: {}", e)
      },
  };

  tcp_stream.write_all(b"Hello server").unwrap();
  tcp_stream.flush().unwrap();

  let mut buffer = [0, 255]; //TODO: možná tady bude problém že se nám přečte jen 255 bytů
  let size = tcp_stream.read(&mut buffer).unwrap();
  let message = String::from_utf8_lossy(&buffer[..size]);
  println!("Server says: {}", message)
}

//control
#[tauri::command]
fn move_drone(side: String) {
  println!("move {}", side)
}

#[tauri::command]
fn rotate_drone(rot_side: i32) {
  println!("rotate {}", rot_side)
}

#[tauri::command]
fn take_off() {
  println!("take_off");
}

#[tauri::command]
fn land() {
  println!("land");
}

#[tauri::command]
fn move_joystick(coords: [i32; 2]) {
  println!("move_joystick: {} {}", coords[0], coords[1]);
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
  .invoke_handler(tauri::generate_handler![connect_to_drone, move_drone, rotate_drone, take_off, land, move_joystick, check_conn])
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}
