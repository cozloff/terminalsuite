use openxr as xr;

pub const EXTENSION_NAME: &str = "XR_EXT_eye_gaze_interaction";
pub const INTERACTION_PROFILE: &str = "/interaction_profiles/ext/eye_gaze_interaction";
pub const TOP_LEVEL_USER_PATH: &str = "/user/eyes_ext";
pub const GAZE_POSE_PATH: &str = "/user/eyes_ext/input/gaze_ext/pose";

pub struct EyeGazeActions {
    _action_set: xr::ActionSet,
    _gaze_pose: xr::Action<xr::Posef>,
}

impl EyeGazeActions {
    pub fn create(instance: &xr::Instance) -> xr::Result<Self> {
        let action_set = instance.create_action_set("eye_gaze", "Eye Gaze", 0)?;
        let eyes = instance.string_to_path(TOP_LEVEL_USER_PATH)?;
        let gaze_pose =
            action_set.create_action::<xr::Posef>("gaze_pose", "Eye Gaze Pose", &[eyes])?;

        let profile = instance.string_to_path(INTERACTION_PROFILE)?;
        let binding = instance.string_to_path(GAZE_POSE_PATH)?;
        instance.suggest_interaction_profile_bindings(
            profile,
            &[xr::Binding::new(&gaze_pose, binding)],
        )?;

        Ok(Self {
            _action_set: action_set,
            _gaze_pose: gaze_pose,
        })
    }
}
