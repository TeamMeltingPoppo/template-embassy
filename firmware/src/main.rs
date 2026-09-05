/* マイコン固有の実装はここに書く
 */

#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

mod application;
mod driver;
mod transport;
mod pubsub;

#[rtic::app(device = embassy_stm32, peripherals=true, dispatchers = [EXTI0,EXTI1,EXTI2])]
mod app {
    use super::*;
    use defmt::*;

    use static_cell::StaticCell;

    use embassy_stm32 as hal;

    hal::bind_interrupts!(struct Irqs {
        FDCAN1_IT0 => hal::can::IT0InterruptHandler<hal::peripherals::FDCAN1>;
        FDCAN1_IT1 => hal::can::IT1InterruptHandler<hal::peripherals::FDCAN1>;
        USART3 => hal::usart::BufferedInterruptHandler<hal::peripherals::USART3>;
        GPDMA1_CHANNEL0 => hal::dma::InterruptHandler<hal::peripherals::GPDMA1_CH0>;
        GPDMA1_CHANNEL1 => hal::dma::InterruptHandler<hal::peripherals::GPDMA1_CH1>;
    });

    #[shared]
    struct Shared {}
    #[local]
    struct Local {}

    #[init]
    fn init(_ctx: init::Context) -> (Shared, Local) {
        let mut config = embassy_stm32::Config::default();
        {
            use embassy_stm32::rcc::*;
            config.rcc.pll1 = Some(Pll {
                source: PllSource::CSI, // 4MHz
                prediv: PllPreDiv::DIV1,
                mul: PllMul::MUL108,
                divp: Some(PllDiv::DIV2), //216MHz
                divq: Some(PllDiv::DIV9), // 48MHz
                divr: None,
            });
            config.rcc.csi = true;
            config.rcc.mux.fdcan12sel = mux::Fdcansel::PLL1_Q;
            config.rcc.sys = Sysclk::PLL1_P; //
        }
        let p = embassy_stm32::init(config);

        let led = hal::gpio::Output::new(p.PA5, hal::gpio::Level::High, hal::gpio::Speed::High);

        let mut config = hal::usart::Config::default();
        config.baudrate = 115200;

        static RX_BUF: StaticCell<[u8; 2048]> = StaticCell::new();
        static TX_BUF: StaticCell<[u8; 512]> = StaticCell::new();
        let rx_buf = &mut RX_BUF.init([0u8; 2048])[..];
        let tx_buf = &mut TX_BUF.init([0u8; 512])[..];

        let uart3 =
            hal::usart::BufferedUart::new(p.USART3, p.PA3, p.PA4, tx_buf, rx_buf, Irqs, config).unwrap();
        // Break serial in TX and RX (not used)
        let (uart_tx, uart_rx) = uart3.split();

        let mut can = hal::can::CanConfigurator::new(p.FDCAN1, p.PC6, p.PC7, Irqs);

        can.properties().set_extended_filter(
            hal::can::filter::ExtendedFilterSlot::_0,
            hal::can::filter::ExtendedFilter::accept_all_into_fifo1(),
        );

        // nominal : 250k bps
        can.set_bitrate(250_000);
        // data : 1M bps
        can.set_fd_data_bitrate(1_000_000, false);

        // nominal : 250k bps
        can.set_bitrate(250_000);
        // data : 1M bps
        can.set_fd_data_bitrate(1_000_000, false);

        info!("Configured");

        let can = can.start(hal::can::OperatingMode::NormalOperationMode);

        let (can_tx, can_rx, _props) = can.split();

        static UART_CHANNEL: thingbuf::mpsc::StaticChannel<pubsub::Item, 5> =
            thingbuf::mpsc::StaticChannel::<pubsub::Item, 5>::new();
        let (uart_channel_sender, uart_channel_receiver) = UART_CHANNEL.split();
        static CAN_CHANNEL: thingbuf::mpsc::StaticChannel<pubsub::Item, 5> =
            thingbuf::mpsc::StaticChannel::<pubsub::Item, 5>::new();
        let (can_channel_sender, can_channel_receiver) = CAN_CHANNEL.split();

        let mut publisher_can_rx_task = pubsub::Publisher::new();
        let mut publisher_uart_rx_task = pubsub::Publisher::new();
        publisher_can_rx_task
            .add_subscriber(uart_channel_sender.clone())
            .unwrap();
        publisher_uart_rx_task
            .add_subscriber(can_channel_sender.clone())
            .unwrap();

        can_rx_task::spawn(can_rx, led, publisher_can_rx_task).ok();
        can_tx_task::spawn(can_tx, can_channel_receiver).ok();
        uart2_rx_task::spawn(uart_rx, publisher_uart_rx_task).ok();
        uart2_tx_task::spawn(uart_tx, uart_channel_receiver).ok();
        task1::spawn().ok();
        (Shared {}, Local {})
    }

    #[task(priority = 1)]
    async fn task1(_ctx: task1::Context) {
        application::task1().await;
    }

    #[task(priority = 3)]
    async fn can_rx_task(
        _ctx: can_rx_task::Context,
        rx: hal::can::CanRx<'static>,
        downlink_led: hal::gpio::Output<'static>,
        publisher: pubsub::Publisher,
    ) {
        transport::can::can_rx_task(rx,downlink_led,publisher).await;
    }

    #[task(priority = 2)]
    async fn can_tx_task(
        _ctx: can_tx_task::Context,
        tx: hal::can::CanTx<'static>,
        receiver: pubsub::MAVLinkReceiver,
    ) {
        transport::can::can_tx_task(tx, receiver).await;
    }

    #[task(priority = 3)]
    async fn uart2_rx_task(
        _ctx: uart2_rx_task::Context,
        rx: hal::usart::BufferedUartRx<'static>,
        publisher: pubsub::Publisher,
    ) {
        transport::uart::uart_rx_task(rx, publisher).await;
    }

    #[task(priority = 2)]
    async fn uart2_tx_task(
        _ctx: uart2_tx_task::Context,
        tx: hal::usart::BufferedUartTx<'static>,
        receiver: pubsub::MAVLinkReceiver,
    ) {
        transport::uart::uart_tx_task(tx, receiver).await;
    }
}
