// ## Temperature & Humidity

// There are two sensors on the I2C bus for internal and external temperature and humidity sensing.

// Uses [SHT31](https://sensirion.com/media/documents/213E6A3B/63A5A569/Datasheet_SHT3x_DIS.pdf)

// ## Orientation

// Uses [BNO055](https://cdn-shop.adafruit.com/datasheets/BST_BNO055_DS000_12.pdf) over the I2C bus at either `0x28` or `0x29` controlled by `J3_ADDR1` in the fenix schematic.

// There is an additional gyro reset control pin connected to the ESP32-S3's GPIO 36.
