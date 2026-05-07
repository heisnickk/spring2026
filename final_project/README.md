# Concurrent Task Dispatcher in Rust

## Project Overview

This project implements a concurrent task scheduling system in Rust that simulates an operating-system-style dispatcher. Tasks are generated over time, placed into queues, and executed by a fixed-size worker pool using different scheduling policies.

The system is designed to demonstrate core systems concepts including concurrency, synchronization, scheduling trade-offs, and performance evaluation.

The project includes two scheduler implementations:

* A FIFO (baseline) scheduler
* An optimized scheduler using separated queues and improved dispatch logic

The behavior of both systems is compared using identical workloads and performance metrics.

---

## How to Build and Run

### Build the project

```bash
cargo build
```

### Run FIFO version

```bash
cargo run --bin fifo
```

### Run Optimized version

```bash
cargo run --bin optimized
```


---

## System Architecture

The system consists of four major components:

* Task Generator
* Dispatcher (Scheduler)
* Worker Pool
* Metrics Collector

### Execution Flow

```
Task Generator → Queue(s) → Dispatcher → Worker Pool → Completion Tracker → Metrics
```

Tasks are generated over time, placed into one or more queues, and assigned to workers based on the scheduling policy. Workers execute tasks concurrently and report completion to a metrics system.

---

## Task Model

Each task contains the following fields:

* id: Unique identifier
* arrival_time: Time the task enters the system
* kind: CPU-bound or IO-bound task
* duration: Simulated execution time

The CPU/IO distinction is used by the optimized scheduler to improve scheduling decisions.

---

## Scheduling Policies

### FIFO Scheduler (Baseline)

The FIFO scheduler processes tasks strictly in order of arrival using a single queue.

Characteristics:

* Simple first-in-first-out ordering
* No distinction between CPU and IO tasks
* Equal treatment of all tasks

Limitations:

* Long CPU tasks can delay short IO tasks
* Poor responsiveness under skewed workloads
* No workload awareness

---

### Optimized Scheduler

The optimized scheduler improves performance using:

* Separate CPU and IO queues
* Priority-based or weighted dispatch logic
* Improved task selection strategy

Characteristics:

* IO tasks can be prioritized for responsiveness
* CPU and IO workloads are separated
* Reduced contention and improved fairness

Trade-offs:

* Increased complexity
* Potential starvation if imbalance is extreme without safeguards

---

## Concurrency Design

The system uses Rust concurrency primitives:

* Threads: used for workers and task generation
* Arc: shared ownership of queues across threads
* Mutex: safe mutable access to shared queues
* Channels (mpsc): used for task completion tracking

### Thread Roles

* Generator Thread: produces tasks over time
* Worker Threads: execute tasks from queues
* (Optional) Dispatcher Logic: manages scheduling decisions

---

## Metrics Collected

The system tracks the following performance metrics:

### Required Metrics

* Total tasks completed
* Makespan (total execution time)
* Average wait time
* Average turnaround time

### Additional Metrics

* Worker utilization
* Maximum wait time
* CPU vs IO task completion counts

These metrics are used to evaluate scheduling efficiency and fairness.

---

## Experiments

### Experiment A — FIFO Baseline

* Balanced workload (CPU and IO mixed)
* Moderate task durations
* FIFO scheduling policy

Observed behavior:
FIFO provides simplicity and fairness by arrival order but suffers under mixed workloads where long CPU tasks delay IO tasks, increasing average wait times.

----

### Experiment B — Optimized Scheduler

* Same or comparable workload as FIFO experiment
* Optimized scheduling policy with separate queues.

Observed behavior:
The optimized scheduler reduces IO latency and improves responsiveness by separating task types and prioritizing execution more intelligently. This leads to improved average wait time and better workload balance.

----

## Tool Use Disclosure

Tools Used:

* ChatGPT was used to assist with system design structure, concurrency architecture, and Rust implementation guidance.
* Rust documentation was used for understanding standard library concurrency primitives.

Example of advice accepted:

* Using separate CPU and IO queues to improve scheduling clarity and performance.

Example of advice modified or rejected:

* Suggestion to use a full async runtime (Tokio) was rejected to keep the design aligned with course requirements and explicit threading concepts.

---

## Lessons Learned

* Designing concurrent systems requires careful ownership and synchronization planning
* Scheduling policy choices directly affect fairness and performance
* FIFO scheduling is simple but inefficient under mixed workloads
* Clean shutdown in multithreaded systems requires explicit coordination
* Rust’s ownership model helps prevent race conditions but increases design complexity

---

## Conclusion

This project demonstrates a concurrent scheduling system with two different scheduling strategies. By comparing FIFO and an optimized scheduler under identical workloads, the impact of scheduling decisions on performance, fairness, and responsiveness becomes clear.
