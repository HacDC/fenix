// [This controller](https://cdn-shop.adafruit.com/datasheets/PCA9685.pdf) is on the I2C bus and can control up to 16 servos.

// The address is controlled by the daughterboard's solder jumpers, and defaults to `0x20` if left open. This is subject to change.

use crate::fenix::fenix_sd::{
    FenixDirectory,
    FenixFile,
};
use embedded_hal::i2c::I2c;

pub struct Servo<'a> {
    // TODO: this is way gross; especially if/when we need to create other
    // persistent variables. Create a separate KVS module?
    is_popped: bool,
    is_popped_file: FenixFile<'a>,
}

impl<'a> Servo<'a> {
    pub fn new(dir: &'a FenixDirectory, fname: &str, _i2c: &mut impl I2c) -> Self {
        // TODO: document the binary structure of this file?
        // Actually: instead of storing data in the file, we could just create the file when we pop?
        let is_popped_file = dir
            .open_file_in_dir(fname, embedded_sdmmc::Mode::ReadWriteCreateOrAppend)
            .unwrap();

        let mut bytes: [u8; 1] = [0];

        // TODO: this unwrap_or is bad? The correct solution is to check if the file exists before we open it?
        let num_bytes = is_popped_file.read(&mut bytes).unwrap_or(1);
        assert!(num_bytes == 1);

        let byte = bytes[0];
        let is_popped: bool = byte != 0;

        // TODO: don't assume that we haven't popped yet?
        Servo {
            is_popped,
            is_popped_file,
        }
    }

    pub fn is_popped(&self, _i2c: &mut impl I2c) -> bool {
        self.is_popped
    }

    pub fn pop(&mut self, _i2c: &mut impl I2c) {
        self.is_popped = true;
        let bytes: [u8; 1] = [1];
        self.is_popped_file.write(&bytes).unwrap();
        todo!();
    }

    pub fn take_picture1(&mut self, _i2c: &mut impl I2c) {
        todo!();
    }

    pub fn take_picture2(&mut self, _i2c: &mut impl I2c) {
        todo!();
    }
}
