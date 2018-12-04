use std::thread;
use std::time::Duration;

fn main(){
    // We create a mutable array of fixed-size array of size 3 filled with 0's
    let mut array: [i32; 3] = [0; 3];

    // We can change the values of the array at specific indices
    array[1] = 1;
    array[2] = 2;

    // We can use the assert_eq macro to ensure that two expressions are equal to each other
    assert_eq!([1, 2], &array[1..]);

    // Note: the array itself is not iterable, we must use an iterator or a reference
    // In this example I coerce the array into a slice by calling a slice method.
    // The final output will be: 0 1 2
    for x in array.iter() { 
        // We can use the print macro to format output
        print!("{} ", x);  
    }

    // vec is a module from the standard library that allows us to 
    // create a contiguous growable array type
    let mut myVec = vec![];

    // Rust's standard library comes with support for spawning native threads
    for _i in 1..10 {
      myVec.push(thread::spawn(|| {my_thread();}));
    }

   println!("main() is waiting.");

    // We can wait for the completion of all the child threads by joining
   for child in myVec {
      match child.join() {
         Ok(s) => (s),
         Err(why) => println!("Failed: {:?}", why),
      };
   }
}

// Output which thread is currently processing and wait for 1 millisecond
fn my_thread() {
   println!("{:?} is processing", std::thread::current().id());
   thread::sleep(Duration::from_millis(1));
}
