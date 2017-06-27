use std::sync::Mutex;

fn main() {
  let mutexed_number = Mutex::new(5);

  {
    let number = mutexed_number.lock().unwrap();
    println!("1 mutexed number plus one equals {}", *number + 1);
  }

  let number = mutexed_number.lock().unwrap();
  println!("2 mutexed number plus one equals {}", *number + 1);
}