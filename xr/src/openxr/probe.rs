use openxr as xr;

use crate::{
    config::AppConfig,
    openxr::eye_gaze::{self, EyeGazeActions},
};

pub struct ProbeReport {
    runtime_name: String,
    runtime_version: xr::Version,
    system_name: Option<String>,
    orientation_tracking: Option<bool>,
    position_tracking: Option<bool>,
    primary_stereo_views: Vec<xr::ViewConfigurationView>,
    extension_names: Vec<String>,
    eye_gaze_supported: bool,
    eye_gaze_action_ready: bool,
}

impl ProbeReport {
    pub fn print(&self) {
        println!("TerminalSuite XR probe");
        println!("runtime: {} v{}", self.runtime_name, self.runtime_version);

        match &self.system_name {
            Some(name) => println!("system: {name}"),
            None => println!("system: no head-mounted display system found"),
        }

        if let Some(orientation_tracking) = self.orientation_tracking {
            println!("orientation tracking: {orientation_tracking}");
        }
        if let Some(position_tracking) = self.position_tracking {
            println!("position tracking: {position_tracking}");
        }

        println!(
            "{}: {}",
            eye_gaze::EXTENSION_NAME,
            if self.eye_gaze_supported {
                "supported"
            } else {
                "not supported"
            }
        );
        println!(
            "eye gaze action binding: {}",
            if self.eye_gaze_action_ready {
                "ready"
            } else {
                "not created"
            }
        );

        if !self.primary_stereo_views.is_empty() {
            println!("primary stereo view configuration:");
            for (index, view) in self.primary_stereo_views.iter().enumerate() {
                println!(
                    "  view {index}: recommended {}x{}, max {}x{}, samples {}",
                    view.recommended_image_rect_width,
                    view.recommended_image_rect_height,
                    view.max_image_rect_width,
                    view.max_image_rect_height,
                    view.recommended_swapchain_sample_count,
                );
            }
        }

        println!("extensions ({}):", self.extension_names.len());
        for name in &self.extension_names {
            println!("  - {name}");
        }
    }
}

pub fn run(config: &AppConfig) -> Result<ProbeReport, Box<dyn std::error::Error>> {
    let entry = load_entry()?;
    #[cfg(target_os = "android")]
    entry.initialize_android_loader()?;

    let available_extensions = entry.enumerate_extensions()?;
    let extension_names = extension_names(&available_extensions);

    let mut enabled_extensions = xr::ExtensionSet::default();
    enabled_extensions.ext_eye_gaze_interaction = available_extensions.ext_eye_gaze_interaction;
    #[cfg(target_os = "android")]
    {
        enabled_extensions.khr_android_create_instance =
            available_extensions.khr_android_create_instance;
    }

    let instance = entry.create_instance(
        &xr::ApplicationInfo {
            application_name: &config.app_name,
            application_version: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap_or(0),
            engine_name: "terminalsuite-xr",
            engine_version: 1,
            api_version: xr::Version::new(1, 0, 0),
        },
        &enabled_extensions,
        &[],
    )?;
    let properties = instance.properties()?;

    let (system_name, orientation_tracking, position_tracking, primary_stereo_views) =
        match instance.system(xr::FormFactor::HEAD_MOUNTED_DISPLAY) {
            Ok(system) => {
                let system_properties = instance.system_properties(system)?;
                let views = instance
                    .enumerate_view_configuration_views(
                        system,
                        xr::ViewConfigurationType::PRIMARY_STEREO,
                    )
                    .unwrap_or_default();

                (
                    Some(system_properties.system_name),
                    Some(system_properties.tracking_properties.orientation_tracking),
                    Some(system_properties.tracking_properties.position_tracking),
                    views,
                )
            }
            Err(_) => (None, None, None, Vec::new()),
        };

    let eye_gaze_action_ready = if available_extensions.ext_eye_gaze_interaction {
        match EyeGazeActions::create(&instance) {
            Ok(actions) => {
                let _keep_alive = actions;
                true
            }
            Err(err) => {
                eprintln!("could not create eye gaze action binding: {err}");
                false
            }
        }
    } else {
        false
    };

    Ok(ProbeReport {
        runtime_name: properties.runtime_name,
        runtime_version: properties.runtime_version,
        system_name,
        orientation_tracking,
        position_tracking,
        primary_stereo_views,
        extension_names,
        eye_gaze_supported: available_extensions.ext_eye_gaze_interaction,
        eye_gaze_action_ready,
    })
}

fn load_entry() -> Result<xr::Entry, Box<dyn std::error::Error>> {
    let entry = unsafe { xr::Entry::load() }.map_err(|err| {
        format!(
            "could not load the OpenXR loader: {err}. Is an OpenXR runtime installed and active?"
        )
    })?;
    Ok(entry)
}

fn extension_names(extensions: &xr::ExtensionSet) -> Vec<String> {
    extensions
        .names()
        .into_iter()
        .map(|name| String::from_utf8_lossy(name).into_owned())
        .collect()
}
