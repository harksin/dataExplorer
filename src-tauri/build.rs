use std::env;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Command;

fn main() {
    Command::new("typeshare")
        .args(&[
            "./",
            "--lang=typescript",
            "--output-file=../src/bindings/commands.ts",
        ])
        .exec();

    tauri_build::build()
}
