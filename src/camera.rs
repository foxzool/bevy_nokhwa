use bevy::prelude::Component;
use flume::unbounded;
use image::RgbaImage;
use nokhwa::pixel_format::{RgbAFormat, RgbFormat};
use nokhwa::utils::{ApiBackend, CameraIndex, RequestedFormat, RequestedFormatType};
use nokhwa::CallbackCamera;
use nokhwa::{nokhwa_initialize, query};

#[derive(Component)]
pub struct BackgroundCamera {
    pub rx: flume::Receiver<RgbaImage>,
}

impl BackgroundCamera {
    pub fn auto() -> Self {
        Self::new(ApiBackend::Auto, None, None)
    }

    pub fn new(
        api: ApiBackend,
        index: Option<CameraIndex>,
        request_format_type: Option<RequestedFormatType>,
    ) -> Self {
        nokhwa_initialize(|granted| {
            println!("User said {granted}");
        });
        let cameras = query(api).unwrap();
        cameras.iter().for_each(|cam| println!("{cam:?}",));
        let (sender, receiver) = unbounded();
        let first_camera = cameras.first().expect("camera not exist");

        let format = RequestedFormat::new::<RgbFormat>(
            request_format_type.unwrap_or(RequestedFormatType::AbsoluteHighestFrameRate),
        );

        let first_camera_index: CameraIndex = match index {
            None => first_camera.index().clone(),
            Some(index) => index,
        };

        let send_fn = move |buffer: nokhwa::Buffer| {
            let image = buffer.decode_image::<RgbAFormat>().unwrap();
            let _ = sender.send(image);
        };

        let mut threaded = CallbackCamera::new(first_camera_index, format, send_fn).unwrap();

        threaded.open_stream().unwrap();

        std::thread::spawn(move || {
            #[allow(clippy::empty_loop)]
            loop {
                threaded.poll_frame().expect("camera error");
            }
        });

        Self { rx: receiver }
    }
}
