use std::sync::{Arc, Mutex};

mod message_passing;
mod shared_state;
mod threads;

pub fn threads() {
    threads::create();
    threads::move_closure();
}

pub fn message_passing() {
    message_passing::channel();
    message_passing::channel_waiting();
    message_passing::clone_channel();
}
pub fn concurrency() {
    shared_state::mutex();
    shared_state::shared_mutex();
    // todo: write a deadlocks by Mutex<T>/Arc<T>
}
