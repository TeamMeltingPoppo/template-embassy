use crate::pubsub;
use embedded_io_async::{Read,Write};
use mavlink::dialects::swingby::MavMessage;

pub async fn uart_rx_task<R>(
    mut rx: R,
    publisher: pubsub::Publisher,
)where R:Read {
    defmt::debug!("rx task started");

    loop {
        // Read raw message to reduce firmware flash size (using read_v2_msg_async will be add ~80KB because
        // all *_DATA::deser methods will be add to firmware).
        let raw = mavlink::read_v2_raw_message_async::<MavMessage>(&mut rx).await;
        match raw {
            Ok(msg) => {
                defmt::debug!(
                    "UART recieved raw message: msg_id={} sys_id={} comp_id={}",
                    msg.message_id(),
                    msg.system_id(),
                    msg.component_id()
                );
                publisher.publish(msg, embassy_time::Instant::now());
            }
            Err(_) => {}
        }
    }
}

pub async fn uart_tx_task<W>(
    mut tx: W,
    receiver: pubsub::MAVLinkReceiver,
) where W: Write {
    // Main loop
    loop {
        if let Some(item) = receiver.recv().await {
            tx.write_all(item.message.raw_bytes()).await.unwrap();
        }
    }
}
