pub fn check_if_cuda() -> bool {
    use std::process::Command;
    let output = Command::new("nvcc").arg("--version").output();
    match output {
        Ok(_) => true,
        Err(_) => false,
    }
}
pub fn check_if_metal() -> bool {
    metal::Device::system_default().is_some()
}
