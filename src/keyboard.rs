use arbitrary_int::u6;
use wasm_bindgen::{JsValue, JsCast};

pub(super) struct Keyboard {
  pub current_scan_code: Option<u6>,
  pending_click_var: wasm_bindgen::JsValue,
}

impl Keyboard {
  pub(super) fn new() -> Self {
    let pending_click_var = js_sys::Reflect::get(
        &wasm_bindgen::JsValue::from(web_sys::window().unwrap()),
        &wasm_bindgen::JsValue::from("getPendingClick"),
    ).unwrap();
    
    Self {
      current_scan_code: None,
      pending_click_var,
    }
  }
  
  pub(super) fn run_refresh_cycle(&mut self) {
    self.handle_keypress();
  }
  
  fn handle_keypress(&mut self) {
    if self.current_scan_code.is_none() {
      if let Some(scan_code) = self.get_keypress() {
        self.current_scan_code = Some(scan_code);
      }
    } else {
      self.current_scan_code = None;
    }
  }
  
  fn get_keypress(&self) -> Option<u6> {
    let pending_click_func: &js_sys::Function = self.pending_click_var.dyn_ref().unwrap();
    let click_var = pending_click_func.apply(&JsValue::null(), &js_sys::Array::new()).unwrap();
    if let Some(click_float) = click_var.as_f64() {
      let code = click_float.round(); //Wish there were a way to get an integer directly without needing to go through a float...
      if code == -1.0 { //Hack around javascript not able to handle 0.
        Some(u6::new(0))
      } else {
        Some(u6::new(code as u8))
      }
    } else {
      None
    }
  }
}
