
use std::sync::mpsc;
use std::thread;

fn main() {
   let (tx, rx) = mpsc::channel();

   thread::spawn(move || {
      for i in [1, 2, 3, 4, 5] {
         tx.send(i).unwrap();
      }
   });

   while let Ok(i) = rx.recv() {
      println!("{}", i);
   }
}
