use ::log::info;
use ::std::fmt::Debug;
use ::std::sync::Arc;
use ::tokio::sync::mpsc;

#[derive(Debug)]
pub struct Source<T: Debug> {
    name: Arc<String>,
    channel: mpsc::Receiver<T>,
}

#[derive(Debug)]
pub struct Sink<T: Debug> {
    name: Arc<String>,
    channel: mpsc::Sender<T>,
}

impl <T: Debug> Sink<T> {
    pub fn fork(&self) -> Sink<T> {
        Sink {
            name: self.name.clone(),
            channel: self.channel.clone(),
        }
    }

    pub fn fork_named(&self, name: impl Into<String>) -> Sink<T> {
        Sink {
            name: Arc::new(name.into()),
            channel: self.channel.clone(),
        }
    }
}

pub fn channel<T: Debug>(size: usize, name: impl Into<String>) -> (Sink<T>, Source<T>) {
    let (sender, receiver) = mpsc::channel(size);
    let name = name.into();
    (
        Sink { name: Arc::new(name.clone()), channel: sender, },
        Source { name: Arc::new(name), channel: receiver, },
    )
}

impl<T: Debug> Drop for Source<T> {
    fn drop(&mut self) {
        info!("Closing channel {:?}", self.name);
    }
}