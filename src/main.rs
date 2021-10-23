mod thread_pool;
mod worker;
mod message;

fn main() {
    let pool = thread_pool::ThreadPool::new( 4);
    for i in 0..10 {
        std::thread::sleep(std::time::Duration::from_millis(250 * i));
       
        pool.spawn(move || {
            println!("This is Task {}", i);
        });

    }
    std::thread::sleep(std::time::Duration::from_secs(2));
}
