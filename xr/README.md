# TerminalSuite XR

Rust-first OpenXR prototype for eye-gaze interaction experiments.

## Run

```sh
cargo run
```

The default mode probes the active OpenXR runtime and prints:

- runtime name/version
- head-mounted display system details
- tracking support
- primary stereo view configuration
- supported extensions
- `XR_EXT_eye_gaze_interaction` support
- whether the eye-gaze pose action binding can be created

## Current Goal

Keep the first slice intentionally narrow:

1. Load the OpenXR runtime.
2. Check whether `XR_EXT_eye_gaze_interaction` is supported.
3. Create the `/user/eyes_ext/input/gaze_ext/pose` action binding.
4. Add a session loop that reads gaze pose data.
5. Add graphics only after the input path is proven.

## Important Runtime Note

Raw OpenXR needs an active OpenXR loader/runtime on the machine running this
binary. If `cargo run` prints `could not load the OpenXR loader`, install or
activate a runtime such as SteamVR, Monado, or the vendor runtime for your
headset, then run the probe again.

## Quest APK Build

This project is configured for `cargo-apk` as an Android `cdylib`.

Known-good local build environment:

```sh
export PATH="/opt/homebrew/opt/rustup/bin:/Users/coz/.cargo/bin:$PATH"
export ANDROID_HOME="/Users/coz/Library/Android/sdk"
export ANDROID_NDK_ROOT="/Users/coz/Library/Android/sdk/ndk/27.2.12479018"
```

Build:

```sh
cargo apk build --lib
```

Install:

```sh
adb install -r target/debug/apk/TerminalSuiteXR.apk
```

Launch:

```sh
adb shell monkey -p com.terminalsuite.xr -c android.intent.category.LAUNCHER 1
```
