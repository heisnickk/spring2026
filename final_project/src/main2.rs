use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{Duration, Instant};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;


// CLEAN SHUTDOWN PATTERN (commented out)

// use std::sync::atomic::{AtomicBool, Ordering};
//
// After setting up queues and before spawning workers, add:
//   let shutdown = Arc::new(AtomicBool::new(false));
//
// Pass a clone into each worker:
//   let shutdown_flag = Arc::clone(&shutdown);
//
// Inside each worker, replace:
//   loop {
// with:
//   while !shutdown_flag.load(Ordering::Relaxed) {
//
// After metrics collection is complete (after the while-let loop), add:
//   shutdown.store(true, Ordering::Relaxed);


#[derive(Clone, Debug)]
enum TaskKind {
    CPU,
    IO,
}

#[derive(Clone, Debug)]
struct Task {
    id: usize,
    kind: TaskKind,
    arrival: Instant,
    duration: Duration,
    start: Option<Instant>,
    finish: Option<Instant>,
}

fn main() {
    let start_sim = Instant::now();

    let cpu_queue = Arc::new(Mutex::new(Vec::<Task>::new()));
    let io_queue = Arc::new(Mutex::new(Vec::<Task>::new()));

    let (done_tx, done_rx) = mpsc::channel::<Task>();

    let total_tasks = 1000;

    // =====================
    // GENERATOR
    // =====================
    {
        let cpu_q = Arc::clone(&cpu_queue);
        let io_q = Arc::clone(&io_queue);

        thread::spawn(move || {
            let mut rng = StdRng::seed_from_u64(42);

            for i in 0..total_tasks {
                let task = Task {
                    id: i,
                    kind: if rng.gen_bool(0.7) { TaskKind::IO } else { TaskKind::CPU },
                    arrival: Instant::now(),
                    duration: Duration::from_millis(rng.gen_range(10..50)),
                    start: None,
                    finish: None,
                };

                match task.kind {
                    TaskKind::CPU => cpu_q.lock().unwrap().push(task),
                    TaskKind::IO => io_q.lock().unwrap().push(task),
                }

                thread::sleep(Duration::from_micros(500));
            }
        });
    }

    // =====================
    // WORKERS
    // =====================
    let workers = 10;

    for _ in 0..workers {
        let cpu_q = Arc::clone(&cpu_queue);
        let io_q = Arc::clone(&io_queue);
        let tx = done_tx.clone();

        thread::spawn(move || {
            loop {
                // If using shutdown pattern, replace `loop {` with:
                // while !shutdown_flag.load(Ordering::Relaxed) {

                let mut task_opt = None;

                // OPTIMIZATION: Prioritize IO tasks over CPU tasks.
                // IO tasks typically wait on hardware (disk, network), so
                // servicing them first reduces system-wide blocking and
                // keeps overall throughput higher compared to naive FIFO.
                {
                    let mut io = io_q.lock().unwrap();
                    if !io.is_empty() {
                        task_opt = Some(io.remove(0));
                    }
                }

                if task_opt.is_none() {
                    let mut cpu = cpu_q.lock().unwrap();
                    if !cpu.is_empty() {
                        task_opt = Some(cpu.remove(0));
                    }
                }

                if let Some(mut task) = task_opt {
                    task.start = Some(Instant::now());

                    thread::sleep(task.duration);

                    task.finish = Some(Instant::now());

                    tx.send(task).unwrap();
                } else {
                    thread::sleep(Duration::from_millis(1));
                }
            }
        });
    }

    drop(done_tx);

    // =====================
    // METRICS
    // =====================
    let mut completed = 0;
    let mut cpu_count = 0;
    let mut io_count = 0;

    let mut wait_total = 0u128;
    let mut turnaround_total = 0u128;
    let mut max_wait = 0u128;

    while let Ok(task) = done_rx.recv() {
        completed += 1;

        match task.kind {
            TaskKind::CPU => cpu_count += 1,
            TaskKind::IO => io_count += 1,
        }

        let wait = task.start.unwrap().duration_since(task.arrival).as_millis();
        let turnaround = task.finish.unwrap().duration_since(task.arrival).as_millis();

        wait_total += wait;
        turnaround_total += turnaround;

        if wait > max_wait {
            max_wait = wait;
        }

        if completed >= total_tasks {
            break;
        }
    }

    // If using shutdown pattern, add here:
    // shutdown.store(true, Ordering::Relaxed);

    let total_runtime = start_sim.elapsed().as_millis();

    println!("== OPTIMIZED simulation ==");
    println!("{} tasks, 70% IO / 30% CPU, {} workers", total_tasks, workers);

    println!("\n__ results __");
    println!("total runtime        : {} ms", total_runtime);
    println!("tasks completed      : {} (IO={}, CPU={})", completed, io_count, cpu_count);
    println!("avg wait time        : {:.2} ms", wait_total as f64 / completed as f64);
    println!("avg turnaround time  : {:.2} ms", turnaround_total as f64 / completed as f64);
    println!("max wait time        : {} ms", max_wait);
}