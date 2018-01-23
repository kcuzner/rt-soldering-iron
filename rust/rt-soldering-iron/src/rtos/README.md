# RTOS written for Rust

This is not the greatest RTOS implementation, nor is it fully complete. It is
optimized for the following:

 - Low memory footprint compared to other offerings (i.e. FreeRTOS). Optimized
   especially for low RAM usage.
 - Compatibility with Cortex-M0+ devices which do not have a full PRIMASK
   register (precluding the use of other Rust RTOS's such as cortex-m-rtfm).

This does require some inline assembly which has been implemented using the
thumbv6m instruction set.

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

