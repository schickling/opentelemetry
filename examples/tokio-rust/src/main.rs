mod o11y;

use anyhow::Result;
use tokio::{join, sync::broadcast};
use tracing::{info, info_span, instrument, Instrument};

#[tokio::main]
async fn main() -> Result<()> {
    o11y::init()?;
    let result = run().await;
    o11y::ship();

    // Not propagating `result` allows us to make sure that `o11y::ship()` gets called
    // even if `result` is `Err`
    result
}

#[instrument(name = "main", skip_all, err)]
async fn run() -> Result<()> {
    info!("Hello World!");

    let (sender, _) = broadcast::channel::<usize>(5);

    // Start 3 identical (concurrent) tasks. Each of them starts an infinite loop
    // and waits for messages on the channel. If the message they receive is `0`,
    // they break out of the loop and the task ends.

    let mut r1 = sender.subscribe();
    let task1 = tokio::spawn(
        async move {
            loop {
                let msg = r1.recv().await;
                info!(?msg, "Task 1");

                if let Ok(n) = msg {
                    if n == 0 {
                        break;
                    }
                }
            }
        }
        .instrument(info_span!("task1")),
    );

    let mut r2 = sender.subscribe();
    let task2 = tokio::spawn(
        async move {
            loop {
                let msg = r2.recv().await;
                info!(?msg, "Task 1");

                if let Ok(n) = msg {
                    if n == 0 {
                        break;
                    }
                }
            }
        }
        .instrument(info_span!("task2")),
    );

    let mut r3 = sender.subscribe();
    let task3 = tokio::spawn(
        async move {
            loop {
                let msg = r3.recv().await;
                info!(?msg, "Task 1");

                if let Ok(n) = msg {
                    if n == 0 {
                        break;
                    }
                }
            }
        }
        .instrument(info_span!("task3")),
    );

    // Send a few messages on the channel
    _ = sender.send(10);
    _ = sender.send(11);
    _ = sender.send(12);
    _ = sender.send(0);

    // Wait for all tasks to finish
    _ = join!(task1, task2, task3);

    Ok(())
}
