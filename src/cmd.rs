use std::process::{Child, Command, Stdio};

use clap::lazy_static::lazy_static;

lazy_static! {
    static ref FFMPEG_BINARY_NAME: &'static str = if cfg!(target_os = "windows") {
        "ffmpeg.exe"
    } else {
        "ffmpeg"
    };
}

pub fn merge(args: [String; 8]) -> Result<Child, std::io::Error> {
    let cmd = format!("{} {}", *FFMPEG_BINARY_NAME, args.join(" "));

    println!("Calling: '{}' 🚀\n", cmd);
    Command::new(*FFMPEG_BINARY_NAME)
        .args(&args)
        .stdout(Stdio::piped())
        .spawn()
}
