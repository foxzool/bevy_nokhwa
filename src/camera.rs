use anyhow::Result;
use bevy::prelude::Component;
use flume::{bounded, unbounded};
use image::RgbaImage;
use nokhwa::pixel_format::{RgbAFormat, RgbFormat};
use nokhwa::utils::{
    ApiBackend, CameraControl, CameraIndex, ControlValueSetter, KnownCameraControl,
    RequestedFormat, RequestedFormatType,
};
use nokhwa::CallbackCamera;
use nokhwa::{nokhwa_initialize, query};
use std::collections::BTreeMap;

#[derive(Component)]
pub struct BackgroundCamera {
    pub image_rx: flume::Receiver<RgbaImage>,
    pub operation_tx: flume::Sender<CameraOperation>,
    pub known_controls: BTreeMap<KnownCameraControl, CameraControl>,
    pub controls: BTreeMap<KnownCameraControl, ControlValueSetter>,
}

pub enum CameraOperation {
    Control {
        id: KnownCameraControl,
        control: ControlValueSetter,
    },
}

impl BackgroundCamera {
    pub fn auto() -> Result<Self> {
        Self::new(ApiBackend::Auto, None, None)
    }

    pub fn new(
        api: ApiBackend,
        index: Option<CameraIndex>,
        request_format_type: Option<RequestedFormatType>,
    ) -> Result<Self> {
        nokhwa_initialize(|granted| {
            println!("User said {granted}");
        });
        let cameras = query(api)?;
        cameras.iter().for_each(|cam| println!("{cam:?}",));
        let (sender, receiver) = unbounded();
        let (op_tx, op_rx) = bounded(1);
        let first_camera = cameras.first().expect("camera not exist");

        let format = RequestedFormat::new::<RgbFormat>(
            request_format_type.unwrap_or(RequestedFormatType::AbsoluteHighestFrameRate),
        );

        let first_camera_index: CameraIndex = match index {
            None => first_camera.index().clone(),
            Some(index) => index,
        };

        let callback_fn = move |buffer: nokhwa::Buffer| {
            let image = buffer.decode_image::<RgbAFormat>().unwrap();
            let _ = sender.send(image);
        };

        let mut threaded = CallbackCamera::new(first_camera_index, format, callback_fn).unwrap();
        let known_controls = threaded.camera_controls_known_camera_controls().unwrap();
        println!("support controls: {known_controls:#?}");

        threaded.open_stream().unwrap();

        std::thread::spawn(move || {
            #[allow(clippy::empty_loop)]
            loop {
                if let Ok(op) = op_rx.try_recv() {
                    match op {
                        CameraOperation::Control { id, control } => {
                            println!("set control: {id} {control}");
                            threaded
                                .set_camera_control(id, control)
                                .expect("camera error");
                        }
                    };
                };

                threaded.last_frame().expect("camera error");
            }
        });

        let known_controls: BTreeMap<KnownCameraControl, CameraControl> = known_controls
            .into_iter()
            .map(|(k, control)| (k, control))
            .collect();

        let controls = known_controls
            .iter()
            .map(|(k, control)| {
                let value = control.value();
                (*k, value)
            })
            .collect();

        Ok(Self {
            image_rx: receiver,
            operation_tx: op_tx,
            known_controls,
            controls,
        })
    }

    pub fn get_mut_bool_control(&mut self, id: &KnownCameraControl) -> Option<&mut bool> {
        if let Some(ControlValueSetter::Boolean(value)) = self.controls.get_mut(id) {
            Some(value)
        } else {
            None
        }
    }

    pub fn get_mut_i64_control(&mut self, id: &KnownCameraControl) -> Option<&mut i64> {
        if let Some(ControlValueSetter::Integer(value)) = self.controls.get_mut(id) {
            Some(value)
        } else {
            None
        }
    }

    pub fn get_mut_f64_control(&mut self, id: &KnownCameraControl) -> Option<&mut f64> {
        if let Some(ControlValueSetter::Float(value)) = self.controls.get_mut(id) {
            Some(value)
        } else {
            None
        }
    }
}
