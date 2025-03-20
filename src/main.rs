use esp_idf_svc::hal::prelude::*;

mod slint_platform;

slint::include_modules!();

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let p = Peripherals::take().unwrap();

    let touch_i2c = esp_idf_svc::hal::i2c::I2cDriver::new(
        p.i2c0,
        p.pins.gpio8,
        p.pins.gpio9,
        &esp_idf_svc::hal::i2c::config::Config::new().baudrate(400_000.Hz()),
    )
    .unwrap();

    slint_platform::init(touch_i2c);

    MainWindow::new().unwrap().run().unwrap();
}
