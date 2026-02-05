use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use esp_hal:: {
    gpio:: {
        Input,
        InputConfig,
        Level,
        Output,
        OutputConfig
    },
    spi:: {
        master:: {
            Config,
            Spi
        },
        Mode
    }
};
use lora_phy:: {
    iv,
    LoRa,
    sx126x
};

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

impl FenixLoRa {
    pub fn new() -> Self {
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
        let spi_bus = SPI_BUS.init(Mutex::new(spi));
        let spi_device = SpiDevice::new(spi_bus, nss);

        // Initialize LoRa Radio
        let interface_variant = iv::GenericSx126xInterfaceVariant::new(rst, dio1, busy, None, None).unwrap();
        let mut lora = LoRa::new(sx126x::Sx126x::new(spi_device, interface_variant, lora_config::RADIO_CONFIG), false, Delay)
            .await
            .unwrap();
        let mut rx_buffer = [0u8; lora_config::PACKET_CONFIG.length as usize];
        let modulation_config = {
            match lora.create_modulation_params(
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
        let mut tx_packet_config = {
            match lora.create_tx_packet_params(
                lora_config::PACKET_CONFIG.preamble,
                lora_config::PACKET_CONFIG.implicit_header,
                lora_config::PACKET_CONFIG.crc,
                lora_config::PACKET_CONFIG.invert_iq,
                &modulation_config
            ) {
                Ok(config) => config,
                Err(e) => {
                    panic!("Failed to create tx packet params: {:?}", e);
                }
            }
        };
    }
}
    // let tx_buffer = b"dasdjjdjksjdfhs";

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
