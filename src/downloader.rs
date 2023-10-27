use std::process::Command;

pub fn download_youtube_audio(url: &str, output: &str) {
    let command = Command::new("yt-dlp")
        .arg("-x")
        .arg("--audio-format")
        .arg("mp3")
        .arg("-o")
        .arg(output)
        .arg(url)
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8_lossy(&command.stdout);

    println!("{}", output);
}

pub fn download_youtube_video(url: &str, output: &str) {
    let command = Command::new("yt-dlp")
        .arg("-f")
        .arg("bestvideo[ext=mp4]+bestaudio[ext=m4a]/mp4")
        .arg("-o")
        .arg(output)
        .arg(url)
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8_lossy(&command.stdout);

    println!("{}", output);

}

pub fn download_playlist(url: &str, output: &str) {
    let command = Command::new("yt-dlp")
        .arg("-f")
        .arg("bestvideo[ext=mp4]+bestaudio[ext=m4a]/mp4")
        .arg("-o")
        .arg(output)
        .arg(url)
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8_lossy(&command.stdout);

    println!("{}", output);
}

pub fn open_mpv(url: &str) {
    let command = Command::new("mpv")
        .arg(url)
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8_lossy(&command.stdout);

    println!("{}", output);
}
