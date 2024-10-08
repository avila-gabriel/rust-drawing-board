# Rust Drawing Board with Minifb: A quick dive into multi-threading

## What This Is

This toy project is a simple Rust program to experiment with multi-threading using a basic drawing board as the playground. The idea was to get a hands-on feel for Rust’s concurrency features and understand how to manage shared state between threads—all within the span of a morning.

## What I Did

### Starting Simple: Single-Threaded Drawing Board

I began by creating a single-threaded drawing board where you could click and drag the mouse to draw on a window. It used a `Vec<u32>` to represent the canvas pixels, and I handled everything—input, drawing, and display updates—in the main loop. The first version was about getting something basic on the screen, understanding Rust’s syntax, and learning to work with the `minifb` crate.

### Adding Line Interpolation

Next, I realized that fast mouse movements left gaps in the lines. To fix this, I implemented a simple line interpolation using Bresenham’s algorithm. This allowed the program to draw continuous lines, even when the mouse was moving quickly.

### Taking the Plunge: Multi-threading

With the single-threaded version working, I decided to introduce multi-threading. The goal was to offload the drawing process to a separate thread while keeping the main thread focused on handling user input and updating the display.

- **Channels and Shared State**: I used Rust’s `mpsc` channels to communicate between the main thread and the drawing thread. The canvas buffer was shared across threads using `Arc<Mutex<Vec<u32>>>`, allowing safe, synchronized access.

### What I Learned

This project was a straightforward introduction to Rust’s multi-threading. It taught me a lot about handling shared state, managing thread-safe data structures, and ensuring that the program remains responsive even when using multiple threads.

- **Why I Did This**: Parallelism is notoriously tricky because of the need for extensive manual testing to catch elusive bugs. I figured that Rust, with its strict safety guarantees and compile-time checks, would be the perfect language to tackle such a program. With a parallelism and distributed programming class on the horizon, I wanted to get my feet wet beforehand. I’ve always been more of an autodidact, and I know that classes tend to be more effective when I’m already engaged with the topic.
- **Rust’s Concurrency Model**: I got hands-on experience with Rust’s ownership and borrowing rules in a concurrent context.
- **Emergence of the Idea**: The idea to focus on shared state came from an earlier attempt at building a web server in Rust. While the web server handled concurrency by managing multiple requests in parallel, it didn’t involve sharing state between threads. This worked well for that context but felt limited. When I moved on to the drawing board project, I faced a different challenge—sharing and synchronizing access to the canvas buffer across threads. This shift from isolated concurrency to managing shared state was where the real learning happened. My previous experience with game engines, where user input and rendering often run in parallel, made this transition smoother and helped me bootstrap the solution quickly.
- **Rapid Adaptability**: One of the most rewarding aspects of this project was how quickly I could adapt to using Rust for a relatively new domain. Rust’s tooling, like `cargo` for project management, made it easy to experiment and iterate rapidly. Despite being a relatively new language for me, Rust’s clear error messages and strong typing system guided me through potential pitfalls, allowing me to focus on the core logic without getting bogged down by obscure bugs.

## How to Run

1. Make sure Rust is installed on your system.
2. Clone this repo and navigate to the project directory.
3. Build and run the project using `cargo build` and `cargo run`.
