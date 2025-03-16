use std::{thread, time};

// fn main() {
//     for n in 1..=1000 {
//         let mut handlers: Vec<thread::JoinHandle<()>> = Vec::with_capacity(n);
//
//         let start = time::Instant::now();
//         for _m in 0..n {
//             let handle = thread::spawn(|| {
//                 let start = time::Instant::now();
//                 let pause = time::Duration::from_millis(20);
//                 while start.elapsed() < pause {
//                     thread::yield_now();
//                 }
//             });
//             handlers.push(handle);
//         }
//
//         while let Some(handle) = handlers.pop() {
//             handle.join();
//         }
//
//         let finish = time::Instant::now();
//         println!("{}\t{:02?}", n, finish.duration_since(start));
//     }
// }

fn main() {
    let pause = time::Duration::from_millis(20);
    let handle1 = thread::spawn(move || {
        thread::sleep(pause);
    });

    let handle2 = thread::spawn(move || {
        thread::sleep(pause);
    });

    handle1.join();
    handle2.join();
}
