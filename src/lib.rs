//! Each clock cycle ends up taking 280 microseconds. (3.671 kHz)

#![forbid(unsafe_code)]

use wasm_bindgen::prelude::*;

use boards::hp_classic::Board;

mod rom45packed;
mod display;
mod keyboard;
mod side_panel;

const REFRESH_RATE: i32 = 20; //20 milliseconds => display at 50 hz.
const CLOCK_RATE: i32 = 280;  //cpu runs at 280 microseconds per opcode.
const CYCLES: i32 = REFRESH_RATE * 1000 / CLOCK_RATE;

/// Loop {
///   Sleep till the next screen refresh
///   Calculate how much instructions we should have run.
///   Run those instructions.
///   Run instructions until the calculated cycles matches the actual cycles.
///   Run IO instructions which should only work on each refresh
///   Draw everything.
/// }
#[wasm_bindgen]
pub async fn run() {
  std::panic::set_hook(Box::new(console_error_panic_hook::hook)); //Panics appear more descriptive in the browser console.

  let mut board = Board::new(rom45packed::ROM.to_vec());
  let mut side_panel = side_panel::SidePanel::new();
  let keyboard = keyboard::Keyboard::new();
  let mut display = display::Display::new();

  let window = web_sys::window().unwrap();

  loop {
    for _ in 0..CYCLES {
      board.run_cycle();
      display.run_cycle(&board);
    }

    keyboard.run_refresh_cycle(&mut board);
    display.run_refresh_cycle(&board);
    
    side_panel.print_cnt(&board);
    side_panel.print_anr(&board);
    side_panel.print_datastorage(&board);

    sleep(&window, REFRESH_RATE).await;
  }
}

/// A trick to get browsers to "sleep" by awaiting a set_timeout
async fn sleep(window: &web_sys::Window, ms: i32) {
  let promise = js_sys::Promise::new(&mut |resolve, _| {
    window.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
  });
  let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}
