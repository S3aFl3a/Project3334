// Concurrent task dispatcher in Rust....
// This project will simulate a stream of incoming work, place that work into 
// one or more queues, and assign tasks to a bounded worker pool according to a 
// scheduling policy. 

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
//Putting stuff here that is REQUIRED for this assignment (use std)
use std::sync::{mpsc, Arc, Mutex}; // mpsc is for communication over channels...
use std::thread; //for threads JoinHandle/
//use std:: collections::VecDeque;
use std::time::{Duration, Instant}; // for duration
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering}; // safely states thread when to stop for this

// Task dispatcher needs to send tasks, create tasks
// fixed 1000 tasks, randomizer (()), 20ms intervals
// 70/305 distrubution of IO/CPU taks

// There's 8 workers (as shown in assignment...)
const WORKER_COUNT: usize = 8;
//total tasks will be 1000, so...
const TOTAL_TASKS: usize = 1000; 




// Requred feature (Task model)
// As shown in the document, EACH task must include at least-
//id, arrival_time, kindwhere kind is either CPU or IO, duration

//MAY include (if useful) - priority, cpu_cost, dealine,and source.
#[derive(Clone, Copy, Debug)]
enum TaskKind {
    CPU, 
    IO,
}

#[derive(Clone, Debug)]
struct Task {
    id: usize,
    kind: TaskKind,
    arrival_time: Instant,
    duration_ms: u64, 
    cpu_cost: u32,
}

// BARRIER TO MAKE READING EASIER HERE ///////

#[derive(Clone, Copy)]
enum Policy {
    FIFO, // Fifo // 70% IO / 30% CPU
    Optimized, //Optomized 80%
}

// METRICS (make sure it is totalruntime)
// storing info
#[derive(Default)]
struct Metrics {
    completed: usize,
    cpu_completed: usize,
    io_completed: usize,
    total_wait: u128,
    total_turnaround: u128,
}
// producer
fn generator (tx:mpsc::Sender<Task>, done: Arc<AtomicBool>) {
    thread::spawn(move || {
        let mut rng = StdRng::seed_from_u64(42);

        for id in 0..TOTAL_TASKS {
            let roll: f64 = r64 = rng.gen();

            let kind = if roll < 0.3 {
                TaskKind::CPU
            } else {
                TaskKind::IO
            };

            let cpu_cost = match kind {
                TaskKind::CPU => 35,
                TaskKind:: IO => 10,
            };

            let duration = match kind {
                TaskKind::CPU => rng.gen_range(180..260),
                TaskKind::IO => rng.gen_range(120..220),
            };

            let task = Task {
                id, kind, 
                arrival_time: Instant::now(),
                duration_ms: duration,
                cpu_cost,
            };

            tx.send(task).unwrap();

            thread::sleep(Duration::from_millis(10));
        }
        done.store(true, Ordering::Relaxed);
    })
}

// focusing on Queue and its schedule
fn dispatcher(
    rx: mpsc::Receiver<Task>,
    worker_senders: Vec<mpsc::Sender<Task>>,
    policy: Policy,
    done: Src<AtomicBool>,
) {
    thread::spawn(move || {
        let mut rr = 0;

        while !done.load(Ordering::Relaxed) || !rx.is_empty() {
            if let Ok(task) = rx.recv_timeout(Duration::from_millis(10)) {
                match policy{
                    Policy::FIFO => {
                        worker_senders[rr % WORKER_COUNT].send(task).unwrap();
                        rr += 1;
                    }

                    Policy::Optimized => {
                        // CPU tasks will now spread evenly go IO 
                        let target = match task.kind {
                            TaskKind::CPU => 0,
                            TaskKind::IO => rr % WORKER_COUNT,
                        };

                        worker_senders[target].send(task).unwrap();
                        rr += 1;
                    }
                }
            }
        }
    });
}

fn worker(
    id: usize,
    rx: mpsc::Receiver<Task>,
    metrics: Arc<Mutex<Metrices>>,
    shutdown: Arc<AtomicBool>,
){
    thread::spawn(move || {
        while !shutdown.load(Ordering::Relaxed) {
            if let Ok(task) = rx.recv_timeout(Duration::from_millis(50)) {
                let start = Instant::now();
                let wait = start.duration_since(task.arrival_time).as_millis();

                thread;:sleep(Duration::from_millis(task.duration_ms));
                let turaround = Instant:: now().duration_since(task.arrival_time).as_millis();

                let mut m = metrics.lock().unwrap();
                m.completed += 1;
                m.total_wait += wait;
                m.total_turnaround += turnaround;

                match task.kind {
                    TaskKind::CPU => m.cpu_done += 1,
                    TaskKind::IO => m.io_done += 1,
                }
                println!("Worker {} finished task {}", id, task.id);
            }
        }
    });
}

// Experiments
// focused on running the experiments
fn run(policy:Policy) {
    print!("\n"),

    match policy {
        Policy::FIFO => println!("Policy: FIFO"),
        Policy::Optimized => println!("Policy: Optimized"),
    }
    let start_time = Instant::now();
    let (tx,rx) = mpsc::channel::<Task>();

    let mut worker_senders = vec![];
    let mut worker_handles = vec![];

    let metrices = Arc::new(Mutex::new(Metrics::default()));
    let shutdown = Arc::new(AtomicBool::new(false));
    let done = Arc::new(AtomicBool::new(false));

    //workers
    for i in 0..WORKER_COUNT {
        let (wtx, wrx) = mpsc::channel();
        worker_senders.push(wtx);

        let m = metrics.clone();
        let sd = shutdown.clone();

        worker_handles.push(worker(i, wrx, m, sd));
    }
    generator(tx, done.clone());
    dispatcher(rx,worker_senders, policy, done.clone());

    // waiting a bit
    thread::sleep(Duration::from_secs(5));
    shutdown.store(true,Ordering::Relaxed);

    let m = metrics.lock().unwrap();

    let makespan = star_time.elapsed().as_millis();
    println!("\nRESULTS ARE:");
    println!("Tasks completed: {}", m.completed);
    println!("CPU tasks: {}", m.cpu_done);
    println!("IO tasks: {}", m.io_done);

    //progress
}

