use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

use std::mem::size_of;
use std::mem::size_of_val;

fn main() {
  println!("type usize: {}", size_of::<usize>());
  println!("type u8: {}", size_of::<u8>()); // 8
  println!("type f64: {}", size_of::<f64>()); // 64
  println!("value 4u8: {}", size_of_val(&4u8)); // 8
  println!("value 4: {}", size_of_val(&4)); // usize? 64?

  println!("value 'a': {}", size_of_val(&'a')); // 32? 8 * 4
  println!("value \"Hello world\" as a static str slice: {}", size_of_val("Hello world")); // 32 * 11
  println!("value \"Hello world\" as a String: {}", size_of_val(&"Hello world".to_string())); // 32 * 11 + pointer? Solo pointer? usize?

  println!("Cell(4): {}", size_of_val(&Cell::new(4))); // usize? 64? same as &4
  println!("RefCell(4): {}", size_of_val(&RefCell::new(4))); // Same as above + locks

  println!("Rc(4): {}", size_of_val(&Rc::new(4))); // Same as &4 + reference counter
  println!("Rc<RefCell(8)>: {}", size_of_val(&Rc::new(RefCell::new(4)))); // ???
}