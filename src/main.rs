mod clipboard;
mod commands;
mod downloader;

use std::{
    process::Command,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

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
    let mut dir: String;

    if args.video {
        dir = format!("~/Videos/{}", today.format("%Y-%m-%d"));
    } else if args.music {
        dir = format!("~/Music/{}", today.format("%Y-%m-%d"));
    } else {
        println!("Please specify either --video or --music");
        return;
    }

    if let Some(path) = args.path {
        dir = path;
    }

    if !check_if_dir_exists(&dir) && !args.mpv {
        let command = Command::new("mkdir")
            .arg("-p")
            .arg(&dir)
            .output()
            .expect("failed to execute process");

        let output = String::from_utf8_lossy(&command.stdout);

        println!("{}", output);
    }

    let (sender, receiver) = std::sync::mpsc::channel();
    let remaining = Arc::new(AtomicUsize::new(0));

    let producer = std::thread::spawn({
        let remaining = remaining.clone();
        move || loop {
            let clipboard = get_clipboard_and_empty();
            println!("{}", clipboard);

            if clipboard.contains("youtube.com") {
                sender.send(clipboard).unwrap();
                remaining.fetch_add(1, Ordering::SeqCst);
            }

            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    let consumer = std::thread::spawn(move || loop {
        let url = receiver.recv().unwrap();

        let output: String;
        if args.playlist {
            output = format!(
                "{}/%(playlist)s/%(playlist_index)s - %(title)s.%(ext)s",
                dir
            );
        } else if args.music {
            output = format!("{}/%(title)s.%(ext)s", dir);
        } else {
            output = "".to_string();
        }

        if args.video {
            downloader::download_youtube_video(&url, &output);
        } else if args.music {
            downloader::download_youtube_audio(&url, &output);
        } else if args.mpv {
            downloader::open_mpv(&url);
        }

        let remaining_items = remaining.load(Ordering::Relaxed);
        println!("Downloaded {} of {}", url, remaining_items);

        remaining.fetch_sub(1, Ordering::SeqCst);
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}
