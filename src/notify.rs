use winrt_notification::{Duration, Sound, Toast};

pub fn notify(title: &str, message: &str) {
    Toast::new(Toast::POWERSHELL_APP_ID)
        .title(title)
        .text1(message)
        .sound(Some(Sound::Default))
        .duration(Duration::Long)
        .show()
        .ok();
}

/*
error[E0308]: mismatched types
  --> src\screens\home.rs:77:49
   |
77 |                         install::carla::install(state);
   |                         ----------------------- ^^^^^ expected `Arc<Mutex<&mut AppState>>`, found `&mut AppState`
   |                         |
   |                         arguments to this function are incorrect
   |
   = note:         expected struct `std::sync::Arc<std::sync::Mutex<&mut AppState>>`
           found mutable reference `&mut AppState`
note: function defined here
  --> src\install\carla.rs:8:8
   |
 8 | pub fn install(state: Arc<Mutex<&mut AppState>>) {
   |        ^^^^^^^ --------------------------------

warning: variable does not need to be mutable
  --> src\install\carla.rs:15:13
   |
15 |         let mut s = state_clone.lock().unwrap();
   |             ----^
   |             |
   |             help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

error[E0521]: borrowed data escapes outside of function
  --> src\install\carla.rs:14:18
   |
 8 |   pub fn install(state: Arc<Mutex<&mut AppState>>) {
   |                  -----            - let's call the lifetime of this reference `'1`
   |                  |
   |                  `state` is a reference that is only valid in the function body
...
14 |       let handle = thread::spawn(move || loop {
   |  __________________^
15 | |         let mut s = state_clone.lock().unwrap();
16 | |         if s.progress >= 0.2 {
17 | |             break;
...  |
20 | |         thread::sleep(Duration::from_millis(100));
21 | |     });
   | |      ^
   | |      |
   | |______`state` escapes the function body here
   |        argument requires that `'1` must outlive `'static`
*/
