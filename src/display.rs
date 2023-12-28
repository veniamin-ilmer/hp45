use boards::hp_classic::Board;
use chips::shifter;

pub(super) struct Display {
  display: web_sys::Element,
  current_str: String,
}

impl Display {
  pub(super) fn new() -> Self {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let display = document.get_element_by_id("display").unwrap();
    
    Self {
      display,
      current_str: String::from("               "),
    }
  }
  
  pub(super) fn run_refresh_cycle(&mut self, board: &Board) {
    let mut buffer = Vec::with_capacity(15);
    let new_str = if board.anr.display_on {
      let mut a = board.anr.a.clone();
      let mut b = board.anr.b.clone();
      let direction = shifter::Direction::Left;
      for location in 0..14 {
        let mask = b.read_nibble(direction);
        let digit = a.read_nibble(direction);
        buffer.push(match mask.value() {
          9 => ' ',
          _ => if location == 11 || location == 0 { //Signs
            if digit.value() == 9 { '-' } else { ' ' }
          } else {
            (digit.value() + 48) as char
          },
        });
        if mask.value() == 2 {
          buffer.push('.');
        }
        a.shift_with_nibble(direction, digit);
        b.shift_with_nibble(direction, mask);
      }
      buffer.iter().collect::<String>()
    } else {
      String::from("               ")
    };
    if self.current_str != new_str {
      self.display.set_text_content(Some(&new_str));
      self.current_str = new_str.clone();
    }
  }
}