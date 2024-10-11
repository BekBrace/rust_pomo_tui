// Pomodoro Timer 1.3
// Timer control works !!
// Room for improvement - alarm display, flash terminal, display calm music for 5 min break

// Importing necessary modules
use cursive::views::{Dialog, TextView};
use cursive::{Cursive, CursiveExt};
use cursive::view::Nameable;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;


fn main() {
    // Main cursive app
    let mut siv = Cursive::default();
    // Variable declarations
    // 25 min pomodoro duration
    let pomodoro_duration = Arc::new(Mutex::new(25 * 60));
    // 5 min break duration
    let break_duration = Arc::new(Mutex::new(5  * 60));
    // Timer to countdown
    let timer_counter = Arc::new(Mutex::new(*pomodoro_duration.lock().unwrap()));

    // Two Flags
    let is_break_time = Arc::new(Mutex::new(false));
    let is_running = Arc::new(Mutex::new(false));

    // Cloning the variables to pass into threads where they will be accessed (safely) and modified.
    let timer_counter_clone = Arc::clone(&timer_counter); 
    let is_break_time_clone = Arc::clone(&is_break_time); 
    let is_running_clone = Arc::clone(&is_running); 

    // Create TextView to display the time left on the timer, and formatted as MM:SS
    let text_view = TextView::new(format!(
        "Time: {}",format_time(*timer_counter.lock().unwrap())
    ))
    .with_name("timer"); // PRO TIP

    // End of variables declarations.
    
    siv.add_layer(
        Dialog::around(text_view)
            .button("+1 min", {
                let pomodoro_duration = Arc::clone(&pomodoro_duration);
                let timer_counter = Arc::clone(&timer_counter);
                move |s| {
                    let mut duration = pomodoro_duration.lock().unwrap();
                    *duration += 60;
    
                    let mut timer_value = timer_counter.lock().unwrap();
                    *timer_value = *duration;
    
                    s.call_on_name("timer", |view: &mut TextView| {
                        view.set_content(format!("Time: {}", format_time(*duration)));
                    });
                }
            })
            .button("-1 min", {
                let pomodoro_duration = Arc::clone(&pomodoro_duration);
                let timer_counter = Arc::clone(&timer_counter);
                move |s| {
                    let mut duration = pomodoro_duration.lock().unwrap();
                    if *duration > 60 {
                        *duration -= 60;
                    }
    
                    let mut timer_value = timer_counter.lock().unwrap();
                    *timer_value = *duration;
    
                    s.call_on_name("timer", |view: &mut TextView| {
                        view.set_content(format!("Time: {}", format_time(*duration)));
                    });
                }
            })
            .button("Start/Stop", {
                let is_running_clone = Arc::clone(&is_running);
                // let timer_counter_clone = Arc::clone(&timer_counter);
                move |s| {
                    let mut running = is_running_clone.lock().unwrap();
                    *running = !*running;
    
                    if *running {
                        s.call_on_name("timer", |view: &mut TextView| {
                            view.set_content("Timer is running...");
                        });
                    } else {
                        s.call_on_name("timer", |view: &mut TextView| {
                            view.set_content("Timer is paused.");
                        });
                    }
                }
            })
            .button("Quit", |s| s.quit())
            .title("Pomodoro Timer"),
    );
    
    // First Thread for handling the countdown of the timer, and it's gonna be a background thread. 
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1));
    
            let mut time_left = timer_counter_clone.lock().unwrap();
            let mut break_time = is_break_time_clone.lock().unwrap();
            let running = is_running_clone.lock().unwrap();
    
            if *running {
                if *time_left > 0 {
                    *time_left -= 1;
                } else {
                    if *break_time {
                        *time_left = *pomodoro_duration.lock().unwrap();
                        *break_time = false;
                        println!("Back to work!");
                    } else {
                        *time_left = *break_duration.lock().unwrap();
                        *break_time = true;
                        println!("Take a break!");
                    }
                }
            }
        }
    });
    
    
    // Second Thread to refresh the UI every second, ensuring the displayed time stays in sync with the countdown.
    let cb_sink = siv.cb_sink().clone();
    let timer_counter_for_refresh = Arc::clone(&timer_counter);

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
    
        let timer_counter_refresh = Arc::clone(&timer_counter_for_refresh);
        cb_sink
            .send(Box::new(move |s| {
                let time_left = *timer_counter_refresh.lock().unwrap();
                s.call_on_name("timer", |view: &mut TextView| {
                    view.set_content(format!("Time: {}", format_time(time_left)));
                });
            }))
            .unwrap();
    });
    
    siv.run();
    
}


// Helper function to format time in MM:SS format for easier display.
fn format_time(time:usize) -> String{
    let minutes = time / 60;
    let seconds = time % 60;
    format!("{:02}:{:02}", minutes, seconds)
}
