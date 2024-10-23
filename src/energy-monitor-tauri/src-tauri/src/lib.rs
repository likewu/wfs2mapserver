use rand::Rng;
use std::convert::TryInto;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{thread, time::Duration};

use tauri::Manager as tauriManager;
use tauri::Window;
use tauri::Emitter;
//use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

static DEMO: AtomicBool = AtomicBool::new(false);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![init_process, toggle_demo])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn toggle_demo() -> bool {
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
        window.emit("distance_emitter", rng.gen_range(20,500)).ok();
        thread::sleep(Duration::from_millis(100));
      } else {
        let dist=200;
        window.emit("distance_emitter", dist).ok();
      }
    }
  });
}
