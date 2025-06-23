mod jlrs;

use rand::Rng;
use std::convert::TryInto;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{thread, time::Duration};

use tauri::Manager as tauriManager;
use tauri::Window;
use tauri::Emitter;
//use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

static DEMO: AtomicBool = AtomicBool::new(false);

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, toggle_demo, init_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn toggle_demo(window: Window) -> bool {
  let mut rng = rand::thread_rng();

  let dist=rng.gen_range(200..500);
  window.emit("distance_emitter", dist).ok();
  !DEMO
    // Ordering::SeqCst is some low level stuff to make sure it is written consequently in memory
    .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(!x))
    .unwrap()
}

// Introducing Btleplug into Tauri
/*fn get_central(manager: &Manager) -> Adapter {
  let adapters = manager.adapters().unwrap();
  adapters.into_iter().nth(0).unwrap()
}*/

// init a background process on the command, and emit periodic events only to the window that used the command
#[tauri::command]
fn init_process(window: Window) {
  // The Rust thread API expects a fully owned closure by API.
  // So the move forces the closure to take ownership rather than borrowing, to fulfill the API.
  // "Because thread::spawn runs this closure in a new thread,
  // ...we should be able to access our value inside that new thread"
  std::thread::spawn(move || {
    let mut rng = rand::thread_rng();

    loop {
      if DEMO.load(Ordering::SeqCst) {
        window.emit("distance_emitter", rng.gen_range(20..500)).ok();
        thread::sleep(Duration::from_millis(100));
      } else {
        let dist=rng.gen_range(100..500);
        window.emit("distance_emitter", dist).ok();
        thread::sleep(Duration::from_millis(100));
      }
    }
  });
}
