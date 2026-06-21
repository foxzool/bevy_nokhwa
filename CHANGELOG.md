# Changes

## [0.8.0] - 2026-06-20

* bump bevy version to `0.19`
  * Migrate the background renderer from the old node/edge render graph to the new
    camera-driven `Core2d`/`Core3d` schedules (`prepare_background` + `render_background`
    systems running before the main pass).
  * Convert the background render pipeline to a `SpecializedRenderPipeline` keyed on the
    view's output format (`ExtractedView::target_format`) **and MSAA sample count**
    (`Msaa::samples()`), compiled asynchronously via the `PipelineCache`. The webcam shader
    is now loaded as an embedded asset. Including the sample count in the key fixes a
    wgpu validation error ("Incompatible sample count") when MSAA is enabled (the default).
  * Update wgpu-25 render-resource API (`immediate_size`, `multiview_mask`,
    `MipmapFilterMode`, `TexelCopyBufferLayout`, buffer slices).
* examples: switch camera format request to `BackgroundCamera::auto()`. Fixed MJPEG
  640x480@30 is rejected by some devices (e.g. Apple Silicon cameras) with
  "Cannot fulfill request".

## [0.6.0] - 2024-07-05

* bump bevy version to `0.14`

## [0.5.0] - 2024-02-20

* bump bevy version to `0.13`

## [0.4.0] - 2023-11-06

* bump bevy version to `0.12`

## [0.2.0] - 2023-03-13

### Updated

- Upgrade `bevy` to 0.10.0

## [0.1.1] - 2023-01-30

### Added

- Add `setting` example for adjust camera control value;
