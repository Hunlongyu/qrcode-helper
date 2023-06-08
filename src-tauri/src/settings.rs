use auto_launch::*;
use tauri::utils::platform::current_exe;

pub fn auto_launch(enable: bool) -> bool {
    let exe = current_exe().unwrap();
    let exe_path = exe.to_str().unwrap();

    let auto = AutoLaunchBuilder::new()
        .set_app_name("QrCode Helper")
        .set_app_path(exe_path)
        .set_use_launch_agent(false)
        .set_args(&["--minimized"])
        .build()
        .unwrap();

    if enable {
        auto.enable().unwrap();
    } else {
        auto.disable().unwrap();
    }

    let is_enable = auto.is_enabled().unwrap();
    return is_enable;
}
