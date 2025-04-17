use chrono::Local;
use std::{
    sync::mpsc::{self, SendError},
    thread,
    time::Instant,
};

fn main() {
    let (tx, rx) = mpsc::channel::<String>();
    let handles: [_; 10] = core::array::from_fn(|_| child(tx.clone()));
    // since 11 tx are made and 10 are sent to child threads, need to drop the 1 left in main.
    drop(tx);
    handles
        .into_iter()
        .enumerate()
        .for_each(move |(i, handle)| {
            handle
                .join()
                .unwrap_or_else(|e| panic!("thread {i} join error: {e:?}"))
                .unwrap_or_else(|e| panic!("thread {i} send error: {e}"));
        });

    let begin = Instant::now();
    let mut maybe_total_length: Result<u64, String> = Ok(0);
    while let Ok(msg) = rx.recv() {
        match maybe_total_length {
            Err(_) => break,
            Ok(total_length) => {
                let Some(acc) = total_length.checked_add(msg.len() as u64) else {
                    maybe_total_length = Err(String::from("total length overflowed"));
                    break;
                };
                maybe_total_length = Ok(acc);
            }
        }
    }
    let perf = begin.elapsed();
    println!(
        "{}",
        maybe_total_length.map_or_else(
            |error| format!("Error during calculating message length: {error}"),
            |sum| format!("Total length of messages received: {sum}")
        )
    );
    println!(
        "Elapsed {} seconds ({}ms)",
        perf.as_secs(),
        perf.as_millis()
    );
}

fn child(tx: mpsc::Sender<String>) -> thread::JoinHandle<Result<(), SendError<String>>> {
    thread::spawn(move || {
        for i in 1..=1_000_000 {
            tx.send(format!(
                "tid {0:?} msg {i} {1}",
                thread::current().id(),
                Local::now().format("%F %r")
            ))?;
        }
        Ok(())
    })
}
