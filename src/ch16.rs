use std::sync::{Arc, Mutex};

mod message_passing;
mod shared_state;
mod threads;

pub(crate) fn concurrency() {
    threads::create();
    threads::move_closure();

    message_passing::channel();
    message_passing::channel_waiting();
    message_passing::clone_channel();

    shared_state::mutex();
    shared_state::shared_mutex();
    // todo: write a deadlocks by Mutex<T>/Arc<T>
}
