#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use embassy_executor::Spawner;
use embassy_time::{
    Delay,
    Duration,
    Timer
};
use embassy_sync::{
    mutex::Mutex,
    blocking_mutex::raw::CriticalSectionRawMutex
};
use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use log::info;
use esp_hal::{
    clock::CpuClock,
    timer::timg::TimerGroup,
    gpio::{
        OutputConfig,
        InputConfig,
        Input,
        Output,
        Level
    },
    spi::{
        Mode,
        master::{
            Config,
            Spi
        },
    },
    time::Rate,
};
use lora_phy::{
    sx126x,
    LoRa,
    iv
};

use spaceblimp::common::lora_config;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

static SPI_BUS: static_cell::StaticCell<Mutex<CriticalSectionRawMutex, Spi<'static, esp_hal::Async>>> =
    static_cell::StaticCell::new();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // generator version: 1.2.0

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 73744);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    info!("Embassy initialized!");



    // TODO: Spawn some tasks
    let _ = spawner;

    loop {
        // let message = b"Be gay, kill  nazis!";
        // rx_buffer[..message.len()].copy_from_slice(message);
        // match lora.prepare_for_tx(
        //     &modulation_config,
        //     &mut tx_packet_config,
        //     lora_config::LORA_POWER,
        //     &rx_buffer
        // ).await {
        //     Ok(_) => {
        //         info!("LoRa radio initialized for TX!");
        //     }
        //     Err(e) => {
        //         panic!("Failed to prepare LoRa radio for TX: {:?}", e);
        //     }
        // }
        // match lora.tx().await {
        //     Ok(_) => {
        //         info!("LoRa TX successful!");
        //     }
        //     Err(e) => {
        //         panic!("LoRa TX failed: {:?}", e);
        //     }
        // }
        // match lora.sleep(true).await {
        //     Ok(_) => {
        //         info!("LoRa radio put to sleep!");
        //     }
        //     Err(e) => {
        //         panic!("Failed to put LoRa radio to sleep: {:?}", e);
        //     }
        // }
        Timer::after(Duration::from_secs(5)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0/examples
}
