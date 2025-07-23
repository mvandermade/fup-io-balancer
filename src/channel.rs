use crate::global::ChannelKey;
use ::log::info;
use ::std::fmt::Debug;
use ::std::mem;
use ::tokio::sync::mpsc;
use tokio::sync::mpsc::error::TrySendError;

#[derive(Debug)]
pub struct Source<T: Debug> {
    key: ChannelKey,
    // Only swapped for None when exposed
    channel: Option<mpsc::Receiver<T>>,
}

impl<T: Debug> Source<T> {
    pub fn expose_receiver(mut self) -> mpsc::Receiver<T> {
        let mut channel = None;
        mem::swap(&mut self.channel, &mut channel);
        channel.expect("Channel receiver already exposed")
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
        Source { key, channel: Some(receiver), },
    )
}

impl <T: Debug> Sink<T> {
    pub async fn send(&self, value: T) -> Result<(), String> {
        self.channel.send(value).await.map_err(|err| err.to_string())
    }

    pub fn try_send(&self, value: T) -> Result<(), (T, String)> {
        match self.channel.try_send(value) {
            Ok(_) => Ok(()),
            Err(TrySendError::Closed(val)) => Err((val, "backlog closed!".to_owned())),
            Err(TrySendError::Full(val)) => Err((val, "full".to_owned())),
        }
    }
}

impl <T: Debug> Source<T> {
    pub async fn receive(&mut self) -> Option<T> {
        match self.channel.as_mut() {
            Some(chan) => chan.recv().await,
            None => panic!("Channel receiver exposed"),
        }
    }
}

pub trait Fork {
    fn fork(&self) -> Self;
}

impl <T: Debug> Fork for Sink<T> {
    fn fork(&self) -> Sink<T> {
        Sink {
            key: self.key.clone(),
            channel: self.channel.clone(),
        }
    }
}

impl<T: Debug> Drop for Source<T> {
    fn drop(&mut self) {
        if let Some(_) = self.channel.take() {
            info!("Closing channel source: {}", self.key);
        }
    }
}

impl<T: Debug> Drop for Sink<T> {
    fn drop(&mut self) {
        info!("Closing one of channel sinks: {}", self.key);
    }
}
