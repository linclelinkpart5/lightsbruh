use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};

fn main() {
    println!("It's lights, bruh!");
    listen();
}

fn listen() {
    let host = cpal::default_host();

    // Get the input devices.
    let input_device_iter = host.input_devices()
        .expect("Cannot retrieve input devices!");
    println!("Found {} audio input devices.", host.input_devices()
        .expect("Cannot retrieve host input devices!").count());

    // Attempt to find a capture stream.
    let mut input_device = host.default_input_device()
        .expect("Cannot retrieve default input device!");
    for device in input_device_iter {
        let input_device_name = device.name().expect("Cannot retrieve device name!");
        println!("Input device: {}.", input_device_name);
        // TODO:  Change this to be either selectable or detectable.
        let found_input = match input_device_name.as_str() {
            "dsnoop:CARD=Device,DEV=0" => true,
            _ => false,
        };
        if found_input {
            input_device = device;
        }
    }

    println!("Using input device: {}.", input_device.name()
        .expect("Cannot retrieve input device name!"));

    // Display the supported input configurations for reference.
    let supported_input_configs = input_device.supported_input_configs()
        .expect("Error while querying configs!");
    for config in supported_input_configs {
        println!("Found supported input config: {} channels, {}/{} min/max sample rate, {} sample_format.",
            config.channels().to_string(), config.min_sample_rate().0, config.max_sample_rate().0,
            config.sample_format().sample_size().to_string());
    }
    // For now, just use the one.
    let supported_config = input_device.supported_input_configs()
        .expect("Error while querying configs!").next()
        .expect("No supported config!").with_max_sample_rate();

    // Do the thing:  Create an input device stream.
    let stream = input_device.build_input_stream(
        &supported_config.config(),
        move |data: & [i16], _: &cpal::InputCallbackInfo| {
            // React to stream events and read stream data here.
            println!("Read some input stream data: {:?}!", data);
        },
        move |_err| {
            // React to errors here.
            println!("Error reading input stream data!");
        },
    ).expect("Unable to build input stream!");

    println!("Built input stream!  Attempting to listen to input stream...");
    stream.play().expect("Unable to play stream!");

    // Porkchop sandwiches!!
    loop {}
}