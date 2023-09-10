mod clipboard;
mod commands;
mod downloader;

use std::process::Command;

use clap::Parser;
use commands::Cli;

use crate::clipboard::get_clipboard_and_empty;

fn check_if_dir_exists(dir: &str) -> bool {
    let command = Command::new("test")
        .arg("-d")
        .arg(dir)
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8_lossy(&command.stdout);

    output.to_string().contains("true")
}

fn main() {
    let args = Cli::parse();

    let today = chrono::offset::Local::now();
    let dir: String;

    if args.video {
        dir = format!("~/Videos/{}", today.format("%Y-%m-%d"));
    } else if args.music {
        dir = format!("~/Music/{}", today.format("%Y-%m-%d"));
    } else {
        println!("Please specify either --video or --music");
        return;
    }

    if !check_if_dir_exists(&dir) {
        let command = Command::new("mkdir")
            .arg("-p")
            .arg(&dir)
            .output()
            .expect("failed to execute process");

        let output = String::from_utf8_lossy(&command.stdout);

        println!("{}", output);
    }

    let (sender, receiver) = std::sync::mpsc::channel();

    let producer = std::thread::spawn(move || loop {
        let clipboard = get_clipboard_and_empty();
        println!("{}", clipboard);

        if clipboard.contains("youtube.com") {
            sender.send(clipboard).unwrap();
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    });

    let consumer = std::thread::spawn(move || loop {
        let url = receiver.recv().unwrap();
        let output = format!("{}/%(title)s.%(ext)s", dir);

        if args.video {
            downloader::download_youtube_video(&url, &output);
        } else if args.music {
            downloader::download_youtube_audio(&url, &output);
        }

        println!("Downloaded {}", url);
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}
