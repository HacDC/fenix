use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    mutex::Mutex,
};
use embassy_time::Delay;
use esp_hal::{
    Async,
    gpio::{
        Input,
        InputConfig,
        Level,
        Output,
        OutputConfig,
    },
    peripherals::Peripherals,
    spi::{
        Mode,
        master::{
            Config,
            Spi,
        },
    },
    time::Rate,
};
use log::info;
use lora_phy::{
    LoRa,
    RxMode,
    iv::GenericSx126xInterfaceVariant,
    mod_params::{
        ModulationParams,
        PacketParams,
    },
    sx126x::{
        Sx126x,
        Sx1262,
    },
};
use static_cell::StaticCell;

use crate::common::lora_config;

// TODO: Relocate for both Fenix and Fraublucher to use
// TODO: Implement TX/RX functions
// TODO: Find solution for radio async timing

pub struct FenixLoRa<'a> {
    rx_buffer: [u8; lora_config::PACKET_CONFIG.length as usize],
    tx_buffer: [u8; lora_config::PACKET_CONFIG.length as usize],
    radio: LoRa<
        Sx126x<
            SpiDevice<'a, CriticalSectionRawMutex, Spi<'static, Async>, Output<'a>>,
            GenericSx126xInterfaceVariant<Output<'a>, Input<'a>>,
            Sx1262,
        >,
        Delay,
    >,
    modulation_config: ModulationParams,
    tx_packet_config: PacketParams,
    rx_packet_config: PacketParams,
}

impl<'a> FenixLoRa<'a> {
    pub async fn new(
        &mut self,
        peripherals: Peripherals,
        spi_bus: &'static StaticCell<Mutex<CriticalSectionRawMutex, Spi<'static, Async>>>,
    ) {
        let radio_init = esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller");
        let (mut _wifi_controller, _interfaces) =
            esp_radio::wifi::new(&radio_init, peripherals.WIFI, Default::default())
                .expect("Failed to initialize Wi-Fi controller");

        // Initialize LoRa radio
        let nss = Output::new(peripherals.GPIO8, Level::High, OutputConfig::default());
        let sclk = peripherals.GPIO9;
        let pico = peripherals.GPIO10;
        let poci = peripherals.GPIO11;
        let rst = Output::new(peripherals.GPIO12, Level::Low, OutputConfig::default());
        let dio1 = Input::new(peripherals.GPIO14, InputConfig::default());
        let busy = Input::new(peripherals.GPIO13, InputConfig::default());
        let spi = Spi::new(
            peripherals.SPI2,
            Config::default()
                .with_frequency(Rate::from_khz(100))
                .with_mode(Mode::_0),
        )
        .unwrap()
        .with_sck(sclk)
        .with_mosi(pico)
        .with_miso(poci)
        .into_async();

        // Initialize Static SPI Bus
        let radio_spi_bus = spi_bus.init(Mutex::new(spi));
        let spi_device = SpiDevice::new(radio_spi_bus, nss);

        // Initialize LoRa Radio
        let interface_variant =
            GenericSx126xInterfaceVariant::new(rst, dio1, busy, None, None).unwrap();
        self.radio = LoRa::new(
            Sx126x::new(spi_device, interface_variant, lora_config::RADIO_CONFIG),
            false,
            Delay,
        )
        .await
        .unwrap();
        self.modulation_config = {
            match self.radio.create_modulation_params(
                lora_config::MODULATION_CONFIG.spreading_factor,
                lora_config::MODULATION_CONFIG.bandwidth,
                lora_config::MODULATION_CONFIG.coding_rate,
                lora_config::FREQUENCY,
            ) {
                Ok(config) => config,
                Err(e) => {
                    panic!("Failed to create modulation params: {:?}", e);
                }
            }
        };
        self.tx_packet_config = {
            match self.radio.create_tx_packet_params(
                lora_config::PACKET_CONFIG.preamble,
                lora_config::PACKET_CONFIG.implicit_header,
                lora_config::PACKET_CONFIG.crc,
                lora_config::PACKET_CONFIG.invert_iq,
                &self.modulation_config,
            ) {
                Ok(config) => config,
                Err(e) => {
                    panic!("Failed to create tx packet params: {:?}", e);
                }
            }
        };
        self.rx_packet_config = {
            match self.radio.create_rx_packet_params(
                lora_config::PACKET_CONFIG.preamble,        // Preamble Length
                lora_config::PACKET_CONFIG.implicit_header, // Implicit Header
                lora_config::PACKET_CONFIG.length,          // Payload Length
                lora_config::PACKET_CONFIG.crc,             // CRC Disabled
                lora_config::PACKET_CONFIG.invert_iq, // Almost certainly don't want IQ inversion
                &self.modulation_config,
            ) {
                Ok(config) => config,
                Err(e) => {
                    panic!("Failed to create rx packet params: {:?}", e);
                }
            }
        };
    }

    pub async fn transmit<'b>(&mut self, payload: &'b [u8]) {
        self.rx_buffer[..payload.len()].copy_from_slice(payload);
        match self
            .radio
            .prepare_for_tx(
                &self.modulation_config,
                &mut self.tx_packet_config,
                lora_config::LORA_POWER,
                &self.rx_buffer,
            )
            .await
        {
            Ok(_) => {
                info!("LoRa radio initialized for TX!");
            }
            Err(e) => {
                panic!("Failed to prepare LoRa radio for TX: {:?}", e);
            }
        }
        match self.radio.tx().await {
            Ok(_) => {
                info!("LoRa TX successful!");
            }
            Err(e) => {
                panic!("LoRa TX failed: {:?}", e);
            }
        }
        // match self.radio.sleep(true).await {
        //     Ok(_) => {
        //         info!("LoRa radio put to sleep!");
        //     }
        //     Err(e) => {
        //         panic!("Failed to put LoRa radio to sleep: {:?}", e);
        //     }
        // }
    }

    pub async fn receive(&mut self) {
        // TODO: Should add this to new() and leave for continuous listen, then interrupt with tx fun
        match self
            .radio
            .prepare_for_rx(
                RxMode::Continuous,
                &self.modulation_config,
                &self.rx_packet_config,
            )
            .await
        {
            Ok(_) => {
                info!("LoRa radio initialized for RX!");
            }
            Err(e) => {
                panic!("Failed to prepare LoRa radio for RX: {:?}", e);
            }
        }
        match self
            .radio
            .rx(&self.rx_packet_config, &mut self.rx_buffer)
            .await
        {
            Ok((rx_size, _rx_packet_status)) => {
                info!(
                    "Received packet: size={} data={:x?}",
                    rx_size,
                    &self.rx_buffer[..rx_size as usize]
                );
            }
            Err(e) => {
                info!("Failed to receive packet: {:?}", e);
            }
        }
    }
}
