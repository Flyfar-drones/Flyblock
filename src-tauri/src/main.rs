// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//message receivers

//settings
#[tauri::command]
fn greet() {
  println!("I was invoked from JS!");
}

//control
#[tauri::command]
fn move_drone(side: String) {
  println!("move {}", side)
}

#[tauri::command]
fn rotate_drone(rot_side: i32) {
  println!("I was invoked from JS!");
}

#[tauri::command]
fn take_off() {
  println!("I was invoked from JS!");
}

#[tauri::command]
fn land() {
  println!("I was invoked from JS!");
}

#[tauri::command]
fn move_joystick(coords: [i32; 2]) {
  println!("I was invoked from JS!");
}

//project

fn main() {
    tauri::Builder::default()
    //messsage handlers
    .invoke_handler(tauri::generate_handler![greet, move_drone])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
