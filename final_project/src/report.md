# Design Report — Concurrent Task Dispatcher in Rust

---

## 1. Architecture

The program is split into four parts that all run at the same time:

- **Task Generator** — creates tasks and drops them into a queue
- **Queue(s)** — holds tasks until a worker is ready to grab one
- **Worker Pool** — a fixed set of threads that pull tasks and run them
- **Metrics Collector** — sits at the end and tracks all the numbers

The flow goes like this:

```
Task Generator -> Queue(s) -> Workers -> Metrics
```

Everything runs on its own thread so nothing is waiting on anything else to finish before it starts.

---

## 2. Data Structures

The main thing is the **Task struct**. Every task has:

- `id` — just a number so we can tell tasks apart
- `kind` — either CPU or IO
- `arrival` — when the task showed up
- `duration` — how long it takes to run
- `start` / `finish` — timestamps we fill in when a worker picks it up and finishes it

The queues are just `Vec<Task>` wrapped in `Arc<Mutex<...>>`. The Arc lets multiple threads share the queue, and the Mutex makes sure only one thread touches it at a time. Finished tasks get sent back to main through an `mpsc` channel.

---

## 3. Synchronization

Three things keep the threads from stepping on each other:

- **Arc** — lets all the worker threads share the same queue without making copies of it
- **Mutex** — locks the queue so only one worker can grab a task at a time, then unlocks right away so others can go
- **mpsc channel** — workers send finished tasks back to main through this, main collects them on the other end

Workers that find an empty queue just sleep for 1ms and try again instead of hammering the lock over and over.

---

## 4. Scheduling Policy

### FIFO — 6 Workers

One queue, tasks come out in the order they went in. Always grabs the front of the queue with `remove(0)`. Simple and fair, but it doesn't care if a task is short or long, CPU or IO — it just takes whatever's next. That means a slow task can hold up a bunch of faster ones behind it.

### Optimized — 10 Workers

Two queues — one for CPU tasks, one for IO tasks. Workers always check the IO queue first, and only go to the CPU queue if IO is empty. The idea is that IO tasks are usually waiting on something external like a disk read, so getting them done faster keeps things moving. CPU tasks can wait a little longer without causing problems.

---

## 5. Metrics

Both versions track the same stuff at the end:

| Metric | What it means |
|---|---|
| Total tasks completed | How many tasks finished |
| Makespan | Total time the simulation ran |
| Avg wait time | How long tasks sat in the queue before a worker grabbed them |
| Avg turnaround time | Full time from arrival to done |
| Max wait time | The worst wait any single task had |
| CPU / IO count | How many of each type finished |

------

## 6. Experiment Results

Same workload for both: 1000 tasks, 70% IO / 30% CPU so the results are the same every run.

| Metric | FIFO (6 Workers) | Optimized (10 Workers) | Difference |
|---|---|---|---|
| Total Runtime | 4999 ms | 3025 ms | -39% |
| Avg Wait Time | 2174.80 ms | 1182.93 ms | -46% |
| Avg Turnaround | 2204.70 ms | 1212.85 ms | -45% |
| Max Wait Time | 4395 ms | 2385 ms | -46% |
| Tasks Completed | 1000 (IO=704, CPU=296) | 1000 (IO=704, CPU=296) | — |

The optimized version finished about 2 seconds faster and cut wait times almost in half. Two things changed at once though — the scheduling policy and the number of workers — so both played a role in the improvement.

---

## 7. Lessons Learned

- Changing which queue workers check first made a real difference in the numbers. A small policy decision had a big effect across 1000 tasks.
- Rust forces you to be very intentional about how threads share data. It's annoying at first but it also means you basically can't accidentally cause a race condition.
- The lock needs to be released before the task runs, not after. If you held the lock the whole time a task was sleeping, every other worker would be stuck waiting and you'd lose all the benefit of having multiple threads.
- A proper shutdown signal (like an `AtomicBool` flag) is the right way to stop workers cleanly. Right now the workers just get killed when main exits, which works fine for this project but wouldn't hold up in a real system. The code has this pattern commented out with notes on how to plug it in.
- Using a fixed seed (`seed_from_u64(42)`) meant every test run produced the exact same tasks, which made comparing the two schedulers actually meaningful.