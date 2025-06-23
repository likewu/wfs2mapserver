use rand::Rng;
use std::convert::TryInto;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{thread, time::Duration};

use tauri::Manager as tauriManager;
use tauri::Window;
use tauri::Emitter;
//use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

use jlrs::prelude::*;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn toggle_demo(window: Window) -> bool {
  /*julia.local_scope::<_, 1>(|mut frame| {
    let _v = Value::new(&mut frame, 1usize);

    // let _v2 = Value::new(&mut frame, 2usize);
  });*/

  true
}

#[tauri::command]
fn init_process(window: Window) {
  let mut julia = Builder::new().start_local().unwrap();

  unsafe {
      julia
          .using("LinearAlgebra")
          //.using("Plots")
          .expect("LinearAlgebra package does not exist");
  }

  julia.local_scope::<_, 1>(|mut frame| {
      let lin_alg = Module::package_root_module(&frame, "LinearAlgebra");
      assert!(lin_alg.is_some());

      let mul_mut_func = lin_alg.unwrap().global(&mut frame, "mul!");
      assert!(mul_mut_func.is_ok());
  })
}
