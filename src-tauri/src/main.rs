// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::borrow::Borrow;
use std::io::{Read, Write};
use std::net::{TcpStream};

//variables
const ADDR: &str = "127.0.0.1";
const PORT: &str = "9001";
#[derive(Debug)]
enum TcpOutput {
    Out1(TcpStream),
    Out2(bool)
}


//message receivers

//settings
#[tauri::command]
fn connect_to_drone() {
  println!("I was invoked from JS!");

  //TCP communication init
  let result = TcpStream::connect(format!("{ADDR}:{PORT}"));
  let tcp_stream = match result {
      Ok(tcp_stream) => TcpOutput::Out1(tcp_stream),
      Err(e) => TcpOutput::Out2(false),
  };
  TcpOutput::borrow(&self)

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

//project

fn main() {
  tauri::Builder::default()
  //messsage handlers
  .invoke_handler(tauri::generate_handler![connect_to_drone, move_drone, rotate_drone, take_off, land, move_joystick])
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}
