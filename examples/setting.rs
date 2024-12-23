extern crate core;

use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::{EguiContexts, EguiPlugin};
use bevy_nokhwa::camera::{BackgroundCamera, CameraOperation};
use bevy_nokhwa::nokhwa::utils::ApiBackend;
use bevy_nokhwa::nokhwa::utils::FrameFormat;
use bevy_nokhwa::nokhwa::utils::{CameraFormat, RequestedFormatType, Resolution};
use bevy_nokhwa::BevyNokhwaPlugin;
use nokhwa::utils::CameraIndex;
use nokhwa::utils::ControlValueDescription;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Setting".to_string(),
                resolution: [1280., 960.].into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .add_plugins(BevyNokhwaPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, dashboard)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn((
            Camera3d::default(),
            Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ))
        // auto find camera and use the highest resolution
        // .insert(BackgroundCamera::auto())
        .insert(
            BackgroundCamera::new(
                ApiBackend::Auto,
                Some(CameraIndex::Index(0)),
                Some(RequestedFormatType::Closest(CameraFormat::new(
                    Resolution::new(640, 480),
                    FrameFormat::MJPEG,
                    30,
                ))),
            )
            .unwrap(),
        );
}

pub fn dashboard(mut egui_context: EguiContexts, mut q_camera: Query<&mut BackgroundCamera>) {
    let mut camera = q_camera.single_mut();
    let known_controls = camera.known_controls.clone();

    egui::Window::new("Camera Controls").show(egui_context.ctx_mut(), |ui| {
        for (known_control, camera_control) in known_controls.iter() {
            ui.label(format!("{:?}", known_control));
            match camera_control.description() {
                ControlValueDescription::Integer { .. } => {
                    if ui
                        .add(egui::DragValue::new(
                            camera.get_mut_i64_control(known_control).unwrap(),
                        ))
                        .changed()
                    {
                        let _ = camera.operation_tx.try_send(CameraOperation::Control {
                            id: known_control.clone(),
                            control: camera.controls.get(known_control).unwrap().clone(),
                        });
                    };
                }
                ControlValueDescription::IntegerRange { min, max, step, .. } => {
                    if ui
                        .add(
                            egui::Slider::new(
                                camera.get_mut_i64_control(known_control).unwrap(),
                                *min..=*max,
                            )
                            .step_by(*step as f64),
                        )
                        .changed()
                    {
                        let _ = camera.operation_tx.try_send(CameraOperation::Control {
                            id: known_control.clone(),
                            control: camera.controls.get(known_control).unwrap().clone(),
                        });
                    };
                }
                ControlValueDescription::Float { .. } => {
                    if ui
                        .add(egui::DragValue::new(
                            camera.get_mut_f64_control(known_control).unwrap(),
                        ))
                        .changed()
                    {
                        let _ = camera.operation_tx.try_send(CameraOperation::Control {
                            id: known_control.clone(),
                            control: camera.controls.get(known_control).unwrap().clone(),
                        });
                    };
                }
                ControlValueDescription::FloatRange { min, max, step, .. } => {
                    if ui
                        .add(
                            egui::Slider::new(
                                camera.get_mut_f64_control(known_control).unwrap(),
                                *min..=*max,
                            )
                            .step_by(*step),
                        )
                        .changed()
                    {
                        let _ = camera.operation_tx.try_send(CameraOperation::Control {
                            id: known_control.clone(),
                            control: camera.controls.get(known_control).unwrap().clone(),
                        });
                    };
                }
                ControlValueDescription::Boolean { .. } => {
                    if ui
                        .add(egui::Checkbox::new(
                            camera.get_mut_bool_control(known_control).unwrap(),
                            "Checked",
                        ))
                        .changed()
                    {
                        let _ = camera.operation_tx.try_send(CameraOperation::Control {
                            id: known_control.clone(),
                            control: camera.controls.get(known_control).unwrap().clone(),
                        });
                    };
                }

                _ => {
                    ui.label("not add ui yet");
                }
            }
        }
    });
}
