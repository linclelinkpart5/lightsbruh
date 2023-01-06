fn main() {
    println!("Hello, world!");

    // Test some USB audio stuff!
    println!("Testing out audio capabilities...");
    use cpal::traits::{HostTrait, DeviceTrait};
    let host = cpal::default_host();
    let device_iter = host.devices().expect("Cannot retrieve host devices!");
    println!("Found {} audio devices.", host.devices().expect("Cannot retrieve host devices!").count());
    for device in device_iter {
        println!("Device: {}.", device.name().expect("Cannot retrieve device name!"));
    }
}
