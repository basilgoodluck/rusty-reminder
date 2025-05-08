// 🦀 RUST LEARNING REMINDER 9000 🦀
// A silly, long, feature-packed Rust program to haunt you until you learn Rust.
// Pin this to your profile as an eternal reminder. You're welcome.

use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::sync::{Arc, Mutex};
use rand::Rng;

const MAX_WARNINGS: u32 = 10;

#[derive(Debug)]
enum ReminderLevel {
    Gentle,
    PassiveAggressive,
    AggressivelyMotivational,
    ExistentialCrisis,
}

impl fmt::Display for ReminderLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            ReminderLevel::Gentle => "Hey buddy, maybe check out Rust today?",
            ReminderLevel::PassiveAggressive => "Oh, still not learning Rust? Interesting.",
            ReminderLevel::AggressivelyMotivational => "DO IT. LEARN RUST. NOW.",
            ReminderLevel::ExistentialCrisis => "What even *is* a life without Rust?",
        };
        write!(f, "{}", msg)
    }
}

trait ReminderBot {
    fn remind(&self);
}

struct RustReminder<'a> {
    name: &'a str,
    level: ReminderLevel,
    reminder_count: u32,
}

impl<'a> RustReminder<'a> {
    fn escalate(&mut self) {
        self.reminder_count += 1;
        self.level = match self.reminder_count {
            0..=2 => ReminderLevel::Gentle,
            3..=5 => ReminderLevel::PassiveAggressive,
            6..=8 => ReminderLevel::AggressivelyMotivational,
            _ => ReminderLevel::ExistentialCrisis,
        };
    }
}

impl<'a> ReminderBot for RustReminder<'a> {
    fn remind(&self) {
        println!("[{} REMINDER]: {}", self.name, self.level);
    }
}

#[derive(Debug)]
struct RustProcrastinationError;

impl fmt::Display for RustProcrastinationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You still haven't learned Rust. This is a grave error.")
    }
}

impl Error for RustProcrastinationError {}

fn log_reminders(history: &mut HashMap<u32, String>, count: u32, msg: &str) {
    history.insert(count, msg.to_string());
}

fn get_random_delay() -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..5)
}

fn main() {
    println!("🦀 Welcome to the RUST LEARNING REMINDER 9000 🦀");

    let history = Arc::new(Mutex::new(HashMap::new()));
    let reminder_name = "🧠 BrainBot";

    let reminder = Arc::new(Mutex::new(RustReminder {
        name: reminder_name,
        level: ReminderLevel::Gentle,
        reminder_count: 0,
    }));

    let reminder_clone = Arc::clone(&reminder);
    let history_clone = Arc::clone(&history);

    let handle = thread::spawn(move || {
        loop {
            let mut bot = reminder_clone.lock().unwrap();
            bot.remind();
            log_reminders(&mut history_clone.lock().unwrap(), bot.reminder_count, &format!("{}", bot.level));
            bot.escalate();

            if bot.reminder_count > MAX_WARNINGS {
                println!("⚠️ WARNING: You've reached peak procrastination.");
                break;
            }

            thread::sleep(Duration::from_secs(get_random_delay()));
        }
    });

    handle.join().unwrap();

    println!("\n📜 Reminder Log:");
    let hist = history.lock().unwrap();
    for (k, v) in hist.iter() {
        println!("Reminder {}: {}", k + 1, v);
    }

    println!("\n✨ Remember: Learning Rust is not a sprint, it's a systems-programming joyride. ✨");

    match check_procrastination() {
        Ok(_) => println!("You're on your way to Rust mastery! 🦀"),
        Err(e) => eprintln!("💥 Error: {}", e),
    }
}

fn check_procrastination() -> Result<(), RustProcrastinationError> {
    let rng = rand::thread_rng().gen_bool(0.5);
    if rng {
        Err(RustProcrastinationError)
    } else {
        Ok(())
    }
}
