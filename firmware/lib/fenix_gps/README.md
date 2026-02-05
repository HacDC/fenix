# GPS

[Embassy UART example](https://github.com/esp-rs/esp-hal/blob/esp-hal-v1.0.0/examples/async/embassy_serial/src/main.rs)

Recommended to find an existing NMEA parser

We're using [this GPS module](https://www.waveshare.com/wiki/L76K_GPS_Module)

| Pin | Function |
| --- | --- |
| 34  | VGNSS_Ctrl |
| 39  | GNSS_Tx    |
| 38  | GNSS_Rx    |
| 40  | GNSS_Wake  |
| 41  | GNSS_PPS   |
| 42  | GNSS_RST   |
