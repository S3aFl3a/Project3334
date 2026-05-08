# Task Dispatcher

## Project Overview 
This final project impements a concurrent task dispatcher that is simulated using Rust's concurrency tools. The system acts how an operating system scheduler handles incoming tasks then assigns them to a bounded worker pool, and evaluates different scheduling methods. 

This is designed with a scheduler where tasks arrive over time and are classified as CPU or IO bound, which are executed with concurrency by a fixed number of worker threads such as 8.

This focuses on system performances and completion time under Results.

##Build the Project
Make sure Rust and Cargo are installed for this program to run. 
Also make sure it is properly installed in the correcting setting (Windows, Mac, etc.)
When building a project with Rust, use the command 'cargo build' as it is the command used to compile Rust projects.

'cargo run' is the command to run the program. 

## The System (Focus)
For the system configuration, this 'simulation' includes the total tasks of 1000, which will be given to workers to do. There are a number of 8 in worker threads. These includes the random seed, task generation, and the two task types that are CPU-bound and IO-bound.

The project generates these 1000 tasks and used a fixed random seed for continuing these results. With the tasks given, it simulates task arrivals over time, place them in into a queue before executing (where a worker can get) by the 8 worker threads, and dispatches tasks. After every task is finished, the program is notified using Arc<AtomicBool> and shuts down without issues. 

## Workers
- Has a fixed size of 8 worker threads
- Each of these workers listen on its own channel queue (for no conflicting problems)
- Execute tasks (by sleeping | duration_ms)

Every task they do, wait for the next one.

## EXPERIMENT
