# Bevy XReal AR demo

A simple demo using [bevy](https://github.com/bevyengine/bevy) and [ar-drivers-rs](https://github.com/badicsalex/ar-drivers-rs) to showcase camera movement in a 3D world. Based on [this](https://bevyengine.org/examples/3D%20Rendering/generate-custom-mesh/) bevy example.

Should work with XREAL Air, Air 2, and Air 2 Pro. Also probably only works on Mac OS (Windows doesn't work, see [issue](https://github.com/badicsalex/ar-drivers-rs/issues/13)).

## Installation

If you already have a Rust environment set up, you can use the `cargo install` command.

Might have to install Xcode app or Xcode command line tools (run `xcode-select --install`) to get bevy working. Dynamic linking is already configured for bevy in `Cargo.toml`.

## Getting started

Set your XReal display as an extended display (Settings -> Displays -> Use as -> Extended display).

Then launch with `cargo run`, drag the window to the extended display and fullscreen.

## Issues

- Jittering - When moving around there is a good amount of jittering of the rendered image.
- Focus - When clicking off of the game window and on something on a different monitor the rendering becomes blurry and input lag is substantially greater.
- Drifting - There is still drift left/right.
- Tracking - The tracking goes wonky when head is tilted close to 90 degrees. Should probably use a different fusion library than `dcmimu`.

### Notes

The goal was to create an [AR desktop](https://www.xreal.com/experience/?virtual-desktop) application as an alternative to XReal's [Nebula](https://www.xreal.com/app/).

I created this small demo with a lot of help from GitHub Copilot. It was my first time writing Rust, so there may be design flaws or errors in the code. Unfortunately, I don't have enough time to continue working on this project and I also lack experience in this field. Therefore I have decided to open source this code in hope that someone more experienced can make something out of it. I have added some notes below of things I learned whist researching and developing this project:

[This](https://kguttag.com/2023/08/05/apple-vision-pro-part-5a-why-monitor-replacement-is-ridiculous/#rendering-a-dot) article goes into great depth explaining the problems with having virtual displays in VR/AR, the main problem being that rendered text uses "hints" to acheive better antialiasing, and when you project the display in virtual space the pixels no longer align properly which results in text that looks grainy and also shimmers. Locking the roll axis would aleviate a lot of the problems with distortion of text. In an ideal scenario you would have a virtual 1080p display be the exact size of the display output so that the pixels line up perfectly with the 1080p display of the glasses.

VITURE's [SpaceWalker](https://www.reddit.com/r/VITURE/comments/1bl72zb/unlock_the_best_of_your_macbook_spacewalker_for/) app has a great implementation of a dropdown UI which should be way easier to implement than slider style settings that Nebula has.

Running as higher priority (`sudo nice -n -10 cargo run`) might reduce input lag. Hard to tell the difference but it feels slightly better.

Might want to consider using `async-hid` instead of `hidapi` for the driver as a way to fix the jitter, although the loop is already running at 1000 Hz so I'm not too sure if that would help.

Experiment with different Hz for fixed schedule of updating the camera. Rendering PreUpdate (before each frame) seems to create more jittering than using a fixed schedule, but results may vary based on your framerate.

## Useful libraries

- [knoll](https://github.com/gawashburn/knoll) - Tool (written in rust) for manipulating the configuration of macOS displays.

- [ScreenCaptureKit](https://github.com/svtlabs/screencapturekit-rs) - A high-performance screen capture (rust) framework for macOS applications.

- [ar-drivers-rs](https://github.com/badicsalex/ar-drivers-rs) - AR driver library for rust.

- [async-hid](https://github.com/sidit77/async-hid) - A rust library for asynchronously interacting with HID devices.

## Useful resources

- [Bevy cheatbook](https://bevy-cheatbook.github.io/)
- [XReal IMU/MCU protocol writeup](https://voidcomputing.hu/blog/worse-better-prettier/#the-prettier-xreal-air)
- [Spreadsheet of FOVs for most AR glasses](https://docs.google.com/spreadsheets/d/1_Af6j8Qxzl3MSHf0qfpjHM9PA-NdxAzxujZesZUyBs0/htmlview)
- [Bevy OpenXR implementation](https://github.com/awtterpip/bevy_oxr)
- [List of XReal compatible apps and drivers](https://github.com/jakedowns/xreal-webxr?tab=readme-ov-file#projects-using-open-source-xreal-drivers)

## License

Licensed under the MIT license
