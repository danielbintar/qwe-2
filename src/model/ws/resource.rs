use std::sync::{
    Arc,
    Mutex,
    mpsc::{self, Sender, Receiver}
};

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
        let (tx_send, rx_receive) = mpsc::channel();
        let sender = Arc::new(Mutex::new(tx_send));
        let receiver = Arc::new(Mutex::new(rx_receive));
        Self {
            tx: sender,
            rx: receiver
        }
    }
}
