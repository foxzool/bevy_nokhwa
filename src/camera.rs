use bevy::prelude::Component;
use flume::bounded;
use image::RgbaImage;
use nokhwa::pixel_format::{RgbAFormat, RgbFormat};
use nokhwa::utils::{ApiBackend, RequestedFormat, RequestedFormatType};
use nokhwa::CallbackCamera;
use nokhwa::{nokhwa_initialize, query};

#[derive(Component)]
pub struct BackgroundCamera {
    pub rx: flume::Receiver<RgbaImage>,
}

impl BackgroundCamera {
    pub fn new() -> Self {
        nokhwa_initialize(|granted| {
            println!("User said {granted}");
        });
        let cameras = query(ApiBackend::Auto).unwrap();
        cameras.iter().for_each(|cam| println!("{cam:?}",));
        let (sender, receiver) = bounded(1);
        let first_camera = cameras.first().unwrap();

        let format =
            RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

        let send_fn = move |buffer: nokhwa::Buffer| {
            let image = buffer.decode_image::<RgbAFormat>().unwrap();

            let _ = sender.send(image);
        };

        let mut threaded =
            CallbackCamera::new(first_camera.index().clone(), format, send_fn.clone()).unwrap();

        threaded.open_stream().unwrap();

        std::thread::spawn(move || {
            #[allow(clippy::empty_loop)]
            loop {
                threaded.poll_frame().expect("camera error");
                // let image = frame.decode_image::<RgbAFormat>().unwrap();
                // println!("loop");
            }
            // let _ = sender.send(image);
        });

        Self { rx: receiver }
    }
}
