use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use core::time;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let host = cpal::default_host();
    let device = host.default_input_device().expect("Failed to get default input device");
    let config = device.default_input_config().expect("Failed to get default input config");
    let sample_rate = config.sample_rate().0;
    let channels = config.channels();

    let file = File::create("record_hello.wav").unwrap();
    let  mut writer = BufWriter::new(file);

    let stream = device.build_input_stream(
        &config.into(),
        
        move |data: &[u8], _: &cpal::InputCallbackInfo| {
            writer.write_all(data).unwrap();
        },
        move |err| {
            eprintln!("an error occurred on stream: {}", err);
        },
        Some(time::Duration::from_secs(10)),
    
    ).unwrap();

    stream.play().unwrap();

    std::thread::sleep(std::time::Duration::from_secs(5));

    stream.pause().unwrap();
}
