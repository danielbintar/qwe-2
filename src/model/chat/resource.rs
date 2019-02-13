use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::{Sender, Receiver};

pub struct Resource {
    pub tx: Arc<Mutex<Sender<String>>>,
    pub rx: Arc<Mutex<Receiver<String>>>,
}

impl Resource {
    pub fn new(tx: Arc<Mutex<Sender<String>>>, rx: Arc<Mutex<Receiver<String>>>) -> Self {
        Self {
            tx,
            rx,
        }
    }
}
