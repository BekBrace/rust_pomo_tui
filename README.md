This repository contains two projects showcasing the use of the Cursive TUI (Text User Interface) library in Rust:

Cursive Basics: A crash course on using Cursive for creating simple terminal-based interfaces.
Pomodoro TUI Application: A Pomodoro timer written in Rust using the Cursive library, implementing concurrency with Arc and Mutex.
Repository Structure
```bash
rust_pomo_tui/

├── cursive/       # Cursive crash course demonstrating simple TUI elements

├── pomo-tui/      # Pomodoro timer TUI application

└── README.md      # Project documentation
```

# Part 1: Cursive Basics
The cursive/ folder contains a simple project that demonstrates how to create basic terminal interfaces using the Cursive library.

Features:
Dialogs: Display dialog boxes with user interaction.
Buttons: Add buttons and handle button click events.
TextView: Show text in the terminal interface.
Event Handling: Manage basic user input events.
How to Run
To try out the Cursive crash course:

Navigate to the cursive/ directory:

```bash
cd cursive
Run the project:
```

```bash
cargo run
This will launch a simple terminal interface showcasing various elements like dialogs, buttons, and text views.
```

# Part 2: Pomodoro TUI Application

The pomo-tui/ folder contains a Pomodoro timer application built using the Cursive library, along with Rust’s Arc and Mutex for handling concurrency in the timer.

## Features:
Custom Timer Durations: Set your own work and break intervals.
Real-Time Timer: Visual countdown timer displayed in the terminal.
Concurrency with Arc/Mutex: Safe state sharing between threads to manage the timer functionality.
How to Run
Navigate to the pomo-tui/ directory:

```bash
cd pomo-tui
Run the application:
```

```bash
cargo run
This will launch the Pomodoro timer interface in your terminal, where you can start your sessions and track your progress.
```

# Requirements
To build and run these projects, ensure you have the following installed:

Rust (latest stable version)
Cargo (Rust’s package manager)

# Contributing
Feel free to fork this repository, make enhancements, or submit issues if you find bugs or have suggestions!
