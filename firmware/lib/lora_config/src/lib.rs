#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use lora_phy::{
    sx126x,
    mod_params
};

pub const FREQUENCY: u32 = 912000000;

pub const RADIO_CONFIG: sx126x::Config<sx126x::Sx1262> = sx126x::Config {
    chip: sx126x::Sx1262,
    tcxo_ctrl: Some(sx126x::TcxoCtrlVoltage::Ctrl1V7),
    use_dcdc: true,
    rx_boost: true,
};

pub struct ModulationConfig {
    pub spreading_factor: mod_params::SpreadingFactor,
    pub bandwidth: mod_params::Bandwidth,
    pub coding_rate: mod_params::CodingRate,
}

pub const MODULATION_CONFIG: ModulationConfig = ModulationConfig {
    spreading_factor: mod_params::SpreadingFactor::_11,
    bandwidth: mod_params::Bandwidth::_250KHz,
    coding_rate: mod_params::CodingRate::_4_5,
};

pub struct PacketConfig {
    pub preamble: u16,
    pub implicit_header: bool,
    pub length: u8,
    pub crc: bool,
    pub invert_iq: bool,
}

pub const PACKET_CONFIG: PacketConfig = PacketConfig {
    preamble: 12,
    implicit_header: false,
    length: 128,
    crc: true,
    invert_iq: false,
};

pub const LORA_POWER: i32 = 22;
