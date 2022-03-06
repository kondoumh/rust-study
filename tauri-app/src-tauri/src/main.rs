#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{
  sync::{
    atomic::{AtomicUsize, Ordering},
    Mutex,
  },
};

use tauri::State;

struct Counter(AtomicUsize);

struct Client;

impl Client {
  fn send(&self) {}
}

#[derive(Default)]
struct Connection(Mutex<Option<Client>>);

#[tauri::command]
fn connect(connection: State<'_, Connection>) {
  *connection.0.lock().unwrap() = Some(Client {});
}

#[tauri::command]
fn disconnect(connection: State<'_, Connection>) {
  // drop the connection
  *connection.0.lock().unwrap() = None;
}

#[tauri::command]
fn connection_send(connection: State<'_, Connection>) {
  connection
    .0
    .lock()
    .unwrap()
    .as_ref()
    .expect("connection not initialize; use the `connect` command first")
    .send();
}

#[tauri::command]
fn increment_counter(counter: State<'_, Counter>) -> usize {
  counter.0.fetch_add(1, Ordering::Relaxed) + 1
}

fn main() {
  tauri::Builder::default()
    .manage(Counter(AtomicUsize::new(0)))
    .manage(Connection(Default::default()))
    .invoke_handler(tauri::generate_handler![
      increment_counter,
      connect,
      disconnect,
      connection_send
    ])
    .run(tauri::generate_context!(
      "tauri.conf.json"
    ))
    .expect("error while running tauri application");
}
