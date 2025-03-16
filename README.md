Just a little project for experimenting with embedded rust on a ESP32-S3-DevKitC-1.

I've tried to do a little LED blinking on GPIO pin 5 both in "baremetal" (still
use HAL for the main function) as well as completely with the HAL.
Unsurprisingly the latter if a lot easier.

I used esp-generate to create the projects. See [the official documentation for
the
HAL](https://docs.espressif.com/projects/rust/esp-hal/1.0.0-beta.0/esp32/esp_hal/index.html#bare-metal-no_std-hal-for-all-espressif-esp32-devices).

The
[datasheet](https://www.espressif.com/sites/default/files/documentation/esp32-s3_datasheet_en.pdf)
and the [technical reference
manual](https://www.espressif.com/sites/default/files/documentation/esp32-s3_technical_reference_manual_en.pdf)
are also both very useful when trying to configure GPIO pins and disabling
watchdogs.

I've yet to try and control the actual on-board LED on GPIO48
([docs](https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32s3/esp32-s3-devkitc-1/user_guide_v1.0.html))
since that requires pulse width modulation, which I know nothing about. Maybe
in the future...
