use std::thread;
use std::sync::{Arc, Mutex};

fn thread_function( i: i32, sum: Arc<Mutex<i32>> ) {
   let mut current_sum = sum.lock().unwrap();
   *current_sum += i;
}

pub fn sum_from_0_to_num_threads( num: i32 ) -> i32 {
   let sum_mutex = Arc::new(Mutex::new( 0 ));
   let mut handles = vec![];

   for i in 1..num+1 {
      let sum_mutex_clone = sum_mutex.clone();
      let handle = thread::spawn( move || thread_function( i, sum_mutex_clone ) );
      handles.push( handle );
   }

   for handle in handles {
      handle.join().unwrap();
   }

   let x = *sum_mutex.lock().unwrap();x
}