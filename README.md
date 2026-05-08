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
This 'simulation' runs the total tasks of 1000 using 8 worker threads. These includes the random seed, task generation, and the two task types that are CPU-bound and IO-bound.

CPU tasks takes longer (180-260) ms
and IO are known to take shorter (120-220) ms.
With the tasks created, it is sent to a dispatcher through a channel. The dispatcher then assigns the task to one of the eight workers based on the scheduling policy (very important). This is where each worker has its own channel and works independently. After every task is finished, the program is notified using Arc<AtomicBool> and shuts down without issues. 

## Workers
- Has a fixed size of 8 worker threads
- Each of these workers listen on its own channel (for no conflicting problems)
- Execute tasks (by sleeping | duration_ms)
Workers complete tasks by sleeping for a set amount of time (using duration_ms) to simulate doing work. When all 1000 tasks are finished, the program uses shared tracking and the (Arc<AtomicBool>) shutdown signal to stop all the threads without hanging or overloading the system. 
Every task they do, wait for the next one.



## EXPERIMENT
  # Experiment 1 (FIFO)
  Focused on workers receiving tasks evenly over time and providing 'fairness' and a balanced policy across workers.

  # Experiment 2 (OPTIMIZED)
  This experiment doesn't really focus on fairness, but tends to always assign CPU tasks to worker 0 (in this case), a notably difference between FIFO and OPTIMIZED. Any IO tasks are distributed across the remaining workers and shows the performances under a obvious uneven task distribution. 
  
  From the results given by avg wait, turnaround, and total runtime, there is a large difference between these two policies in results. 
  
