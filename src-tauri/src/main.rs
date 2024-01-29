// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//message receivers

//settings
#[tauri::command]
fn connect_to_drone() {
  println!("I was invoked from JS!");
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
