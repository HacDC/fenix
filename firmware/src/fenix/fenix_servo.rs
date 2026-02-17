// [This controller](https://cdn-shop.adafruit.com/datasheets/PCA9685.pdf) is on the I2C bus and can control up to 16 servos.

// The address is controlled by the daughterboard's solder jumpers, and defaults to `0x20` if left open. This is subject to change.

use embedded_hal::i2c::I2c;

pub struct Servo {
    is_popped: bool,
}

impl Servo {
    pub fn new(_i2c: &mut impl I2c) -> Self {
        // TODO: don't assume that we haven't popped yet?
        Servo { is_popped: false }
    }

    pub fn is_popped(&self, _i2c: &mut impl I2c) -> bool {
        self.is_popped
    }

    pub fn pop(&mut self, _i2c: &mut impl I2c) {
        self.is_popped = true;
        todo!();
    }

    pub fn take_picture1(&mut self, _i2c: &mut impl I2c) {
        todo!();
    }

    pub fn take_picture2(&mut self, _i2c: &mut impl I2c) {
        todo!();
    }
}
