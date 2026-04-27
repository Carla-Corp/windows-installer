pub mod carla;
pub mod dependencies;
pub mod finish;
pub mod morgana;

use crate::SET_PROGRESS;

use druid::{ExtEventSink, Target};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};

use std::thread;
use std::time::Duration;

pub fn animate_progress(
    sink: ExtEventSink,
    current: Arc<Mutex<f64>>,
    cap: f64,
    running: Arc<AtomicBool>,
    added: Arc<Mutex<f64>>,
) {
    thread::spawn(move || {
        let mut add = 0f64;
        while running.load(Ordering::Relaxed) {
            {
                add += 0.01;
                let mut p = current.lock().unwrap();
                (*p) += add.min(cap);
                let _ = sink.submit_command(SET_PROGRESS, *p, Target::Auto);
            }
            thread::sleep(Duration::from_millis(300));
        }
        *added.lock().unwrap() = add;
    });
}
