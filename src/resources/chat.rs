use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::{Sender, Receiver};

pub struct Chat {
    pub tx: Arc<Mutex<Sender<String>>>,
    pub rx: Arc<Mutex<Receiver<String>>>,
}

impl Chat {
    pub fn new(tx: Arc<Mutex<Sender<String>>>, rx: Arc<Mutex<Receiver<String>>>) -> Self {
        Self {
            tx,
            rx,
        }
    }
}
