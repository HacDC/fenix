extern crate alloc;
use alloc::boxed::Box;
use embedded_hal_bus::spi::ExclusiveDevice;
use embedded_sdmmc::{
    Directory,
    File,
    SdCard,
    TimeSource,
    Timestamp,
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
    peripherals::{
        GPIO34,
        GPIO35,
        GPIO36,
        GPIO37,
        SPI2,
    },
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

pub struct DummyTimeSource;

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

pub type FenixSDCard =
    SdCard<ExclusiveDevice<SpiMaster<'static, Blocking>, Output<'static>, EspDelay>, EspDelay>;

pub type FenixFile<'a> = File<'a, FenixSDCard, DummyTimeSource, 4, 4, 1>;

pub type FenixDirectory = Directory<'static, FenixSDCard, DummyTimeSource, 4, 4, 1>;

pub struct FenixSDArgs {
    pub spi2: SPI2<'static>,
    pub gpio34: GPIO34<'static>,
    pub gpio35: GPIO35<'static>,
    pub gpio36: GPIO36<'static>,
    pub gpio37: GPIO37<'static>,
}

pub fn open_sd(args: FenixSDArgs) -> FenixDirectory {
    let spi_bus: SpiMaster<'_, Blocking> = SpiMaster::new(
        args.spi2,
        SpiConfig::default()
            .with_frequency(Rate::from_khz(400))
            .with_mode(Mode::_0),
    )
    .unwrap()
    .with_sck(args.gpio36)
    .with_mosi(args.gpio35)
    .with_miso(args.gpio37);
    let sd_chip_select: Output<'_> = Output::new(args.gpio34, Level::High, OutputConfig::default());
    let spi_device: ExclusiveDevice<SpiMaster<'_, Blocking>, Output<'_>, EspDelay> =
        ExclusiveDevice::new(spi_bus, sd_chip_select, EspDelay::new()).unwrap();
    let sdcard = SdCard::new(spi_device, EspDelay::new());
    info!("SD Card initialized: {:?}", sdcard.num_bytes());
    let volume_manager = VolumeManager::new(sdcard, DummyTimeSource);
    let volume_manager = Box::leak(volume_manager.into());

    let volume = volume_manager.open_volume(VolumeIdx(0)).unwrap();
    let volume = Box::leak(volume.into());
    volume.open_root_dir().unwrap()
}
