use core::str as String;
use embedded_hal_bus::spi::ExclusiveDevice;
use embedded_sdmmc::{
    Directory,
    File,
    Mode as FileMode,
    SdCard,
    TimeSource,
    Timestamp,
    Volume,
    VolumeIdx,
    VolumeManager,
};
use esp_hal::{
    delay::Delay as EspDelay,
    gpio::{
        Level,
        Output,
        OutputConfig,
    },
    peripherals::Peripherals,
    spi::{
        master::{
            Config as SpiConfig,
            Spi as SpiMaster,
        },
        Mode,
    },
    time::Rate,
    Blocking,
};
use log::info;

struct DummyTimeSource;

impl TimeSource for DummyTimeSource {
    fn get_timestamp(&self) -> Timestamp {
        Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

pub struct FenixSD<'d> {
    // volume_manager: VolumeManager<SdCard<ExclusiveDevice<SpiMaster<'d, Blocking>, Output<'d>, EspDelay>, EspDelay>, DummyTimeSource>,
    file: File<
        'd,
        SdCard<ExclusiveDevice<SpiMaster<'d, Blocking>, Output<'d>, EspDelay>, EspDelay>,
        DummyTimeSource,
        4,
        4,
        1,
    >,
}

impl<'d> FenixSD<'d> {
    pub async fn new(peripherals: Peripherals) -> Self {
        let spi_bus: SpiMaster<'d, Blocking> = SpiMaster::new(
            peripherals.SPI2,
            SpiConfig::default()
                .with_frequency(Rate::from_khz(400))
                .with_mode(Mode::_0),
        )
        .unwrap()
        .with_sck(peripherals.GPIO36)
        .with_mosi(peripherals.GPIO35)
        .with_miso(peripherals.GPIO37);
        let sd_chip_select: Output<'d> =
            Output::new(peripherals.GPIO34, Level::High, OutputConfig::default());
        let spi_device: ExclusiveDevice<SpiMaster<'d, Blocking>, Output<'d>, EspDelay> =
            ExclusiveDevice::new(spi_bus, sd_chip_select, EspDelay::new()).unwrap();
        // let spi_device = ExclusiveDevice::new(spi_bus, sd_chip_select, Delay);
        let sdcard = SdCard::new(spi_device, EspDelay::new());
        info!("SD Card initialized: {:?}", sdcard.num_bytes());
        let mut volume_manager = VolumeManager::new(sdcard, DummyTimeSource);

        let mut volume = volume_manager.open_volume(VolumeIdx(0)).unwrap();
        let mut root = volume.open_root_dir().unwrap();
        let mut file = root
            .open_file_in_dir(
                String::from_utf8(b"FlyingFenix.log").unwrap(),
                FileMode::ReadWriteCreateOrAppend,
            )
            .unwrap();
        file.write(b"Opened SD\n");

        Self { file }
    }
}
