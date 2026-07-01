use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Instant,Duration};
// various imports needed to establish the righ architecture

pub type task<T> = Box<dyn FnOnce() -> T + Send + 'static>; // 

fn main() {

}
