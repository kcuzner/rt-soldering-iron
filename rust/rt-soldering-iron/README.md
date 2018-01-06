# Firmware for the RT Soldering Iron, written in rust

## Why

Rust is a system-level language which should allow us to retain the
low-levelness of C while adding the following:

- Memory safety
- Some semblance of enforced data safety (see the RTOS design note below)
- All the awesomeness that comes with pattern matching and other language
  features which would be really handy.

This is not without cost, however. I believe that generally this firmware will
have a larger footprint than something equivalent in C (assuming that we don't
use FreeRTOS of course, that thing is huge).

## RTOS Design

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

