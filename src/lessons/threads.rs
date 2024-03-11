use rayon::prelude::*;
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::thread::{self, sleep};
use std::time::Duration;

use fake::faker::name::raw::Name;
use fake::locales::EN;
use fake::Fake;

struct Work {
    name: String,
    done: bool,
}

impl Work {
    fn new() -> Work {
        let new_name = Name(EN).fake();
        Work {
            name: new_name,
            done: false,
        }
    }

    pub fn process(&mut self) {
        sleep(Duration::from_millis(100));
        println!("Processing work: {}", self.name);
        self.done = true;
    }
}

fn process_jobs(recv_channel: Receiver<&mut Work>) {
    // without the par_bridge, the work is done in serial so if we had
    // 100 items then we would expect this to take 100 * 100ms = 10s
    recv_channel.into_iter().par_bridge().for_each(|work| {
        work.process();
    });
}

fn simple_thread() -> thread::JoinHandle<Vec<i32>> {
    let mut v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        v.push(4);

        return v;
    });

    return handle;
}

fn mutex_and_atomic_ref_counts(num_increments: usize) -> i32 {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..num_increments {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    return *counter.lock().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    static mut WORKS: Vec<Work> = vec![];

    #[test]
    fn test_simple_thread() {
        let handle = simple_thread();

        let result = handle.join().unwrap();

        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_simple_channels() {
        // Initialize our work
        for _ in 0..100 {
            unsafe { WORKS.push(Work::new()) };
        }

        // For rayon to use 100 threads, or else Rayon will default to using a pool
        // with the same number of threads as the number of cores on the machine.
        rayon::ThreadPoolBuilder::new()
            .num_threads(100)
            .build_global()
            .unwrap();

        // Create our channel for sending work
        let (send, recv) = mpsc::channel::<&mut Work>();

        // Send our work to the channel for processing
        for work in unsafe { WORKS.iter_mut() } {
            send.send(work).unwrap();
        }
        drop(send); // Indicate that sending is done

        // Process the work will use Rayon to parallelize the work handling and
        // this will block until all work is done
        process_jobs(recv);

        // Ensure all work is done
        for work in unsafe { WORKS.iter() } {
            assert_eq!(work.done, true);
        }
    }

    #[test]
    fn test_mutex_and_atomic_ref_counts() {
        let result = mutex_and_atomic_ref_counts(100);

        assert_eq!(result, 100);
    }
}
