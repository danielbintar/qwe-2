use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

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

impl Default for Resource {
    fn default() -> Self {
        let (_, rx_receive) = mpsc::channel();
        let (tx_send, _) = mpsc::channel();
        let sender = Arc::new(Mutex::new(tx_send));
        let receiver = Arc::new(Mutex::new(rx_receive));
        Self {
            tx: sender,
            rx: receiver
        }
    }
}
