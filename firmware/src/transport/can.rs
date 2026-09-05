use crate::pubsub;
use embassy_stm32::can;
use embedded_hal::digital::OutputPin;
use mavlink::dialects::swingby::MavMessage;

pub async fn can_rx_task<Output>(
    mut rx: can::CanRx<'static>,
    mut downlink_led: Output,
    publisher: pubsub::Publisher,
) where
    Output: OutputPin,
{
    let mut last_read_ts = embassy_time::Instant::now();
    downlink_led.set_low().ok();
    loop {
        match rx.read_fd().await {
            Ok(envelope) => {
                downlink_led.set_high().ok();
                let (ts, rx_frame) = (envelope.ts, envelope.frame);
                let delta = (ts - last_read_ts).as_millis();
                last_read_ts = ts;
                defmt::debug!("CAN Rx: {} --- {}ms", rx_frame.header().len(), delta,);
                let mut buffer = rx_frame.data();
                let msg = mavlink::read_v2_raw_message_async::<MavMessage>(&mut buffer).await;
                if let Ok(msg) = msg {
                    defmt::info!(
                        "CAN recieved raw message: msg_id={} sys_id={} comp_id={}",
                        msg.message_id(),
                        msg.system_id(),
                        msg.component_id()
                    );
                    publisher.publish(msg, embassy_time::Instant::now());
                }
                downlink_led.set_low().ok();
            }
            Err(_err) => defmt::error!("Error in frame"),
        }
    }
}

pub async fn can_tx_task(mut tx: can::CanTx<'static>, receiver: pubsub::MAVLinkReceiver) {
    // Main loop
    loop {
        if let Some(item) = receiver.recv().await {
            let raw_bytes = item.message.raw_bytes();
            let mut buffer: [u8; 64] = [0u8; 64];
            if let Some(len) = pad_can_fd_payload(raw_bytes, &mut buffer) {
                let frame = can::frame::FdFrame::new_extended(
                    item.message.component_id() as u32,
                    &buffer[..len],
                )
                .unwrap();
                _ = tx.write_fd(&frame).await;
            }
        }
    }
}

fn can_fd_len(len: usize) -> usize {
    match len {
        0..=8 => len,
        9..=12 => 12,
        13..=16 => 16,
        17..=20 => 20,
        21..=24 => 24,
        25..=32 => 32,
        33..=48 => 48,
        49..=64 => 64,
        _ => {
            defmt::error!("MAVLink packet is too large for CAN FD (len={})", len);
            0xFF
        }
    }
}

fn pad_can_fd_payload(data: &[u8], buffer: &mut [u8; 64]) -> Option<usize> {
    let len = can_fd_len(data.len());
    if len <= 64 {
        buffer[..len].fill(0);
        buffer[..data.len()].copy_from_slice(data);
        Some(len)
    } else {
        None
    }
}
