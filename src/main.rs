fn main() {
    println!("It's lights, bruh!");
    listen();
}

fn scale_col(value: isize, low: isize, high: isize) -> u8 {
    if value < low {
        return 0;
    }
    if value > high {
        return 255;
    }
    (255 * (value - low) / (high - low)) as u8
}

fn rotate([x, y]: [isize; 2], angle: f64) -> [f64; 2] {
    [
        x as f64 * angle.cos() - y as f64 * angle.sin(),
        x as f64 * angle.sin() + y as f64 * angle.cos(),
    ]
}

fn display() {
    use std::io::Write;
    use rpi_led_panel::{
        RGBMatrix,
        RGBMatrixConfig,
    };

    let config: RGBMatrixConfig = argh::from_env();
    let rows = config.rows as isize;
    let cols = config.cols as isize;
    let (mut matrix, mut canvas) = RGBMatrix::new(config, 0)
        .expect("Matrix initialization failed");

    let [center_x, center_y] = [cols / 2, rows / 2];

    let rotate_square = (rows.min(cols) as f64 * 1.41) as isize;
    let min_rotate = center_x - rotate_square / 2;
    let max_rotate = center_x + rotate_square / 2;

    let display_square = (rows.min(cols) as f64 * 0.7) as isize;
    let min_display = center_x - display_square / 2;
    let max_display = center_x + display_square / 2;

    for step in 0.. {
        let rotation_deg = step as f64 / 2.0;
        for x in min_rotate..max_rotate {
            for y in min_rotate..max_rotate {
                let [rot_x, rot_y] =
                    rotate([x - center_x, y - center_x], rotation_deg.to_radians());
                let canvas_x = rot_x + center_x as f64;
                let canvas_y = rot_y + center_y as f64;
                if (min_display..max_display).contains(&x)
                    && (min_display..max_display).contains(&y)
                {
                    canvas.set_pixel(
                        canvas_x as usize,
                        canvas_y as usize,
                        scale_col(x, min_display, max_display),
                        255 - scale_col(y, min_display, max_display),
                        scale_col(y, min_display, max_display),
                    )
                } else {
                    canvas.set_pixel(canvas_x as usize, canvas_y as usize, 0, 0, 0)
                }
            }
        }

        canvas = matrix.update_on_vsync(canvas);

        if step % 120 == 0 {
            print!("\r{:>100}\rFramerate: {}", "", matrix.get_framerate());
            std::io::stdout().flush().unwrap();
        }
    }
}

fn listen() {
    use cpal::traits::{
        HostTrait,
        DeviceTrait,
        StreamTrait,
    };

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
        println!("Found supported input config: {} channels, \
                {}/{} min/max sample rate, {} sample formats.",
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
            let mut sum = 0.0f32;
            for datum in data {
                sum += *datum as f32;
            }
            let average = sum / data.len() as f32;
            println!("Read some input stream data (average): {:?}!", average);
        },
        move |_err| {
            // React to errors here.
            println!("Error reading input stream data!");
        },
    ).expect("Unable to build input stream!");

    println!("Built input stream!  Attempting to listen to input stream...");
    stream.play().expect("Unable to play stream!");

    // Porkchop sandwiches?!
    display();
}