# RTOS written for Rust

This is not the greatest RTOS implementation, nor is it fully complete. It is
optimized for the following:

 - Low memory footprint compared to other offerings (i.e. FreeRTOS). Optimized
   especially for low RAM usage.
 - Compatibility with Cortex-M0+ devices which do not have a full PRIMASK
   register (precluding the use of other Rust RTOS's such as cortex-m-rtfm).

This does require some inline assembly which has been implemented using the
thumbv6m instruction set.

## Memory Design

In rust, almost everything is allocated on the stack. This means that for each
task, the stack needs to be quite large.

One Rust RTOS design I saw, RTFM by Jorge Aparicio, used the interrupt masking
ability of the Cortex-M3 to have different runtime levels. This made for a very
lightweight RTOS stack, but had several unfortunate limitations:

- It cannot be ported to Cortex-M0[+] or other architectures which do not
  possess a multi-level global interrupt mask in their NVIC.
- It cannot have more tasks than there are interrupt mask levels. On a normal
  Cortex-M3, this is 8 levels.
- Since tasks were associated with hardware interrupts, a pure software task was
  a little less straightforward than with systems like FreeRTOS.

In general, Rust provides the Arc for working with shared things across threads.
Sadly, the Arc allocates the T on the heap, which is not acceptable here. I
would also need to port it since it would have to make use of my RTOS.

One option is to make everything stack allocated and follow that pattern. I
would end up with the following issues:

- Mutexes and such would have a difficult time with their lifetimes, especially
  at the interface between interrupts and tasks.
- I could not statically analyze the memory usage of the program. While I'll
  probably never get fully to this point, this completely removes that ability.
- The list of tasks is very likely to be impossible using only stack
  allocations. Even the SmallVec implementation cheats somewhat by always
  allocating an exact size on the stack by directly declaring an array of a
  specific size.

The zinc implementation solves the stack-list issue in two ways:
- For tasks waiting on Mutexes or Queues, they use a very unsafe queue
  implementation that relies on the fact that each node in the queue is
  allocated in the stack of a task that will be blocked and returning from the
  blocking function only occurs once the node has been dequeued. This is not
  applicable for the vast majority of situations and also forces a
  first-come-first-serve with mutexes rather than making it priority-based.
- For the task list, it is a statically allocated array of a fixed size. I
  dislike this since I haven't yet found an easy way in rust to create the array
  of tasks to be the exact size I need without code modifications.
  - One way around this is to possibly use a macro to generate an exactly sized
    and pre-linked task list, statically allocated.

Another option is to make everything static:

- The `static` scope is ok, but mutable statics are considered unsafe so I'd
  have to implement interior mutability. Or just use cells.
- Statics cannot have the result of a function be their initializing value,
  unless `const fn` is used.

Yet another option is to generate the RTOS on the fly with a macro. However, I
think this is a little more complicated than necessary.

## Principle of Operation

The basic execution unit is a Task. Tasks are created from a function pointer
and a static mutable array used as the stack (this aids in static analysis of
memory usage). Tasks can have one of two states:

 - Ready
 - Blocked

As the RTOS runs, it maintains a list of the tasks with their states. At any
given time, only one task is running. All other tasks are suspended. When a
context switch is requested and the scheduler runs, it will choose the highest
priority `Ready` task and resume it.

All context switches are performed by way of the PendSV interrupt. This
interrupt occurs when a bit is set in the SCB register, but may not occur
immediately at this time.

Since all context switches are by way of an interrupt, they are asynchronous and
a task can be suspended at any point. The program has no control over where the
task will resume.Care must be taken to ensure that functions which request that
the task be blocked do not exit prematurely.

All tasks are maintained in a global static list. This is an optimization to
remove the need for most synchronization entities to maintain and track a
separate list of tasks. When a task is blocked, an identifier is stored which is
used later during the unblock phase to prevent unintentional unblocking of
tasks.

The priority of a task is implemented on a first-come-first-serve basis.
The first created task is the highest priority task and all subsequent tasks
will be at a lower priority. The lowest priority task is the idle task, which is
resumed whenever all application tasks are blocked. The priority of a task
cannot be changed on the fly.

