use boards::hp_classic;
use chips::cpu::hp_anr;
use chips::shifter;

pub struct SidePanel {
  cnt: web_sys::HtmlCollection,
  anr: web_sys::HtmlCollection,
  ram: web_sys::HtmlCollection,
  status: web_sys::HtmlCollection,
  current_anr: [hp_anr::Register; 7],
  current_data: [hp_anr::Register; 10],
  current_status: [bool; 12],
}

impl SidePanel {
  
  pub fn new() -> Self {
    Self {
      cnt: get_tr_list("cnt"),
      anr: get_tr_list("anr"),
      ram: get_tr_list("ram"),
      status: get_tr_list("status"),
      current_anr: Default::default(),
      current_data: Default::default(),
      current_status: Default::default(),
    }
  }
  
  pub fn print_anr(&mut self, board: &hp_classic::Board) {
    let tr_list = &self.anr;
    print_reg(&mut self.current_anr[0], tr_list, board.anr.a, 1);
    print_reg(&mut self.current_anr[1], tr_list, board.anr.b, 2);
    print_reg(&mut self.current_anr[2], tr_list, board.anr.c, 3);
    print_reg(&mut self.current_anr[3], tr_list, board.anr.d, 4);
    print_reg(&mut self.current_anr[4], tr_list, board.anr.e, 5);
    print_reg(&mut self.current_anr[5], tr_list, board.anr.f, 6);
    print_reg(&mut self.current_anr[6], tr_list, board.anr.m, 7);
  }
  
  pub fn print_datastorage(&mut self, board: &hp_classic::Board) {
    let tr_list = &self.ram;
    print_reg(&mut self.current_data[0], tr_list, board.ram.r0, 1);
    print_reg(&mut self.current_data[1], tr_list, board.ram.r1, 2);
    print_reg(&mut self.current_data[2], tr_list, board.ram.r2, 3);
    print_reg(&mut self.current_data[3], tr_list, board.ram.r3, 4);
    print_reg(&mut self.current_data[4], tr_list, board.ram.r4, 5);
    print_reg(&mut self.current_data[5], tr_list, board.ram.r5, 6);
    print_reg(&mut self.current_data[6], tr_list, board.ram.r6, 7);
    print_reg(&mut self.current_data[7], tr_list, board.ram.r7, 8);
    print_reg(&mut self.current_data[8], tr_list, board.ram.r8, 9);
    print_reg(&mut self.current_data[9], tr_list, board.ram.r9, 10);
  }
  
  pub fn print_cnt(&mut self, board: &hp_classic::Board) {
    let td_list = self.cnt.item(1).expect("can't get tr").children();
    if let Some(td) = td_list.item(1) {
      td.set_text_content(Some(&format!("{:04o}", board.cnt.next_address)));
    }
    let td_list = self.cnt.item(2).expect("can't get tr").children();
    if let Some(td) = td_list.item(1) {
      td.set_text_content(Some(&format!("{:04o}", board.cnt.saved_address)));
    }
    let td_list = self.cnt.item(3).expect("can't get tr").children();
    if let Some(td) = td_list.item(1) {
      td.set_text_content(Some(&format!("{:X}", board.cnt.pointer)));
    }
    self.print_status(board);
  }
  
  fn print_status(&mut self, board: &hp_classic::Board) {
    if self.current_status != board.cnt.status {
      let td_list = self.status.item(1).expect("can't get tr").children();
      for i in 0..12 {
        if let Some(td) = td_list.item(i as u32 + 1) {
          if board.cnt.status[i] {
            td.set_text_content(Some("●"));
          } else {
            td.set_text_content(Some("○"));
          }
        }
      }
      self.current_status = board.cnt.status;
    }
  }
}

fn print_reg(current_reg: &mut hp_anr::Register, tr_list: &web_sys::HtmlCollection, mut new_reg: hp_anr::Register, row_index: u32) {
  if new_reg != *current_reg {
    let td_list = tr_list.item(row_index).expect("can't get tr").children();
    for i in 0..14 {
      let col_index = i as u32 + 2;
      let nibble = new_reg.read_nibble(shifter::Direction::Left);
      if let Some(td) = td_list.item(col_index) {
        td.set_text_content(Some(&format!("{:X}", nibble)));
      }
      new_reg.shift_with_nibble(shifter::Direction::Left, nibble);
    }
    *current_reg = new_reg.clone();
  }
}

fn get_tr_list(table_id: &str) -> web_sys::HtmlCollection {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let table = document.get_element_by_id(table_id).expect("can't find table id");
  let tbody = table.children().item(0).expect("can't get tbody");
  tbody.children()
}