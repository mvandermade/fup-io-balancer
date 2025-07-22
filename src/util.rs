use crate::global::ChannelKey;
use ::log::info;
use ::std::fmt::Debug;
use ::tokio::sync::mpsc;

#[derive(Debug)]
pub struct Source<T: Debug> {
    key: ChannelKey,
    channel: mpsc::Receiver<T>,
}

impl<T: Debug> Source<T> {
    pub fn expose_receiver(self) -> mpsc::Receiver<T> {
        let Source { key: _, channel } = self;
        channel
        //TODO @mark: does this prevent drop?
    }
}

#[derive(Debug)]
pub struct Sink<T: Debug> {
    key: ChannelKey,
    channel: mpsc::Sender<T>,
}

pub fn channel<T: Debug>(size: usize, key: ChannelKey) -> (Sink<T>, Source<T>) {
    let (sender, receiver) = mpsc::channel(size);
    (
        Sink { key, channel: sender, },
        Source { key, channel: receiver, },
    )
}

impl <T: Debug> Sink<T> {
    pub async fn send(&self, value: T) -> Result<(), String> {
        self.channel.send(value).await.map_err(|err| err.to_string())
    }
}

impl <T: Debug> Source<T> {
    pub async fn receive(&mut self) -> Option<T> {
        self.channel.recv().await.map(|maybe| maybe)
    }
}

impl <T: Debug> Sink<T> {
    pub fn fork(&self) -> Sink<T> {
        Sink {
            key: self.key.clone(),
            channel: self.channel.clone(),
        }
    }
}

impl<T: Debug> Drop for Source<T> {
    fn drop(&mut self) {
        info!("Closing channel source: {}", self.key);
    }
}

impl<T: Debug> Drop for Sink<T> {
    fn drop(&mut self) {
        info!("Closing one of channel sinks: {}", self.key);
    }
}
