#[cfg(target_os="windows")]
use system_shutdown::shutdown_with_message;

pub fn shutdown_with_message_wrapper(msg: &str, duration: u32, force: bool) -> Result<(), std::io::Error> {
    #[cfg(target_os="windows")]
    return shutdown_with_message(msg, duration, force);

    #[cfg(not(target_os="windows"))]
    return {
        println!("\
Simulating shutdown:
msg: {},
duration: {},
force: {}",
        msg, duration, force);
        Ok(())
    }
}

