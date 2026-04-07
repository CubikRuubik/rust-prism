use colored::*;
use std::error::Error;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::task;
use tokio::time::sleep;

#[derive(Debug, Clone)]
struct Job {
    id: usize,
    value: i32,
}

#[derive(Debug)]
struct Result {
    job_id: usize,
    output: i32,
    took: Duration,
}

fn print_colored(label: &str, message: &str) {
    print!("[{}] ", label.green());
    println!("{}", message.yellow());
}

async fn process(job: Job) -> Result<Result, Box<dyn Error + Send + Sync>> {
    let start = Instant::now();
    // Simulate work by sleeping random duration up to 50 ms
    let sleep_duration = rand::random::<u64>() % 50;
    sleep(Duration::from_millis(sleep_duration)).await;

    if job.value < 0 {
        return Err(format!("Negative value for job {}", job.id).into());
    }

    Ok(Result {
        job_id: job.id,
        output: job.value * job.value,
        took: start.elapsed(),
    })
}

async fn run_pipeline(
    jobs: Vec<Job>,
    workers: usize,
) -> Result<Vec<Result>, Box<dyn Error + Send + Sync>> {
    let (job_tx, mut job_rx) = mpsc::channel::<Job>(jobs.len());
    let (result_tx, mut result_rx) = mpsc::channel::<Result>(jobs.len());

    // Send all jobs
    for job in jobs {
        job_tx.send(job).await.map_err(|_| "Failed to send job")?;
    }
    drop(job_tx); // Close sender

    // Spawn worker tasks
    let mut handles = Vec::new();
    for _ in 0..workers {
        let mut job_rx = job_rx.clone();
        let result_tx = result_tx.clone();
        let handle = task::spawn(async move {
            while let Some(job) = job_rx.recv().await {
                match process(job).await {
                    Ok(res) => {
                        if result_tx.send(res).await.is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            Ok::<(), Box<dyn Error + Send + Sync>>(())
        });
        handles.push(handle);
    }
    drop(result_tx);

    // Await all workers
    for handle in handles {
        handle.await??;
    }

    // Collect results
    let mut results = Vec::new();
    while let Some(res) = result_rx.recv().await {
        results.push(res);
    }

    Ok(results)
}

#[tokio::main]
async fn main() {
    println!("Hello, World!");
    println!("First Commit");
    println!("Second Commit");
    print_colored("info", "colored output via fatih/color");

    let jobs = vec![
        Job { id: 1, value: 3 },
        Job { id: 2, value: 7 },
        Job { id: 3, value: 5 },
        Job { id: 4, value: 2 },
    ];

    match run_pipeline(jobs.clone(), 2).await {
        Ok(results) => {
            for r in results {
                println!("job {}: {}² = {} (took {:?})", r.job_id, jobs[r.job_id - 1].value, r.output, r.took);
            }
        }
        Err(e) => {
            eprintln!("Error running pipeline: {}", e);
            return;
        }
    }
}
