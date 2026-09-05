use mavlink::MAVLinkV2MessageRaw;
use thingbuf::mpsc::{StaticReceiver, StaticSender};

pub struct Item {
    pub timestamp: embassy_time::Instant,
    pub message: MAVLinkV2MessageRaw,
}
impl Default for Item {
    fn default() -> Self {
        Item {
            timestamp: embassy_time::Instant::from_secs(0),
            message: MAVLinkV2MessageRaw::new(),
        }
    }
}
impl Clone for Item {
    fn clone(&self) -> Self {
        Item {
            timestamp: self.timestamp.clone(),
            message: self.message.clone(),
        }
    }
}

pub type MAVLinkSender = StaticSender<Item>;
pub type MAVLinkReceiver = StaticReceiver<Item>;

pub struct Publisher {
    subscribers: [Option<StaticSender<Item>>; 5],
}

impl Publisher {
    pub const fn new() -> Self {
        let subscribers: [Option<MAVLinkSender>; 5] = [None, None, None, None, None];
        Self { subscribers }
    }
    pub fn add_subscriber(&mut self, channel: MAVLinkSender) -> Result<(), ()> {
        while let Some(pos) = self.subscribers.iter().position(|x| x.is_none()) {
            self.subscribers[pos] = Some(channel);
            return Ok(());
        }
        Err(())
    }
    pub fn publish(
        &self,
        message: MAVLinkV2MessageRaw,
        timestamp: embassy_time::Instant,
    ) {
        for sub in self.subscribers.iter() {
            if let Some(channel) = sub {
                let result = channel
                    .try_send(Item {
                        timestamp,
                        message,
                    });
                if let Err(_) = result {
                    defmt::error!("Failed to send message to subscriber");
                }
            }
        }
    }
}