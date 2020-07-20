use tokio::process::Command;

const PATH_1080_60: &str = "1080_60/playlist.m3u8";
const PATH_720_60: &str = "720_60/playlist.m3u8";
const PATH_720_30: &str = "720_30/playlist.m3u8";
const PATH_480_30: &str = "480_30/playlist.m3u8";

pub async fn start() {
    let _ = Command::new("ffmpeg")
        .kill_on_drop(true)
        .args(&["-i", "udp://localhost:2525?fifo_size=114688"])
        .args(&["-c:v", "copy", "-c:a", "copy"])
        .args(&[
            "-f",
            "hls",
            "-hls_init_time",
            "4",
            "-hls_time",
            "4",
            "-hls_list_size",
            "5",
            "-hls_flags",
            "temp_file+delete_segments+independent_segments",
            "-strftime",
            "1",
            "-strftime_mkdir",
            "1"
            PATH_1080_60,
        ])
        .args(&[
            "-c:v",
            "libx264",
            "-x264opts",
            "keyint=120:no-scenecut",
            "-s",
            "1280x720",
            "-r",
            "60",
            "-b:v",
            "4500",
            "-profile:v",
            "main",
            "-preset",
            "veryfast",
            "-c:a",
            "aac",
            "-sws_flags",
            "bilinear",
        ])
        .args(&[
            "-f",
            "hls",
            "-hls_init_time",
            "4",
            "-hls_time",
            "4",
            "-hls_list_size",
            "5",
            "-hls_flags",
            "temp_file+delete_segments+independent_segments",
            "-strftime",
            "1",
            "-strftime_mkdir",
            "1"
            PATH_720_60,
        ])
        .args(&[
            "-c:v",
            "libx264",
            "-x264opts",
            "keyint=60:no-scenecut",
            "-s",
            "1280x720",
            "-r",
            "30",
            "-b:v",
            "3000",
            "-profile:v",
            "main",
            "-preset",
            "veryfast",
            "-c:a",
            "aac",
            "-sws_flags",
            "bilinear",
        ])
        .args(&[
            "-f",
            "hls",
            "-hls_init_time",
            "4",
            "-hls_time",
            "4",
            "-hls_list_size",
            "5",
            "-hls_flags",
            "temp_file+delete_segments+independent_segments",
            "-strftime",
            "1",
            "-strftime_mkdir",
            "1"
            PATH_720_30,
        ])
        .args(&[
            "-c:v",
            "libx264",
            "-x264opts",
            "keyint=60:no-scenecut",
            "-s",
            "852x480",
            "-r",
            "30",
            "-b:v",
            "2000",
            "-profile:v",
            "main",
            "-preset",
            "veryfast",
            "-c:a",
            "aac",
            "-sws_flags",
            "bilinear",
        ])
        .args(&[
            "-f",
            "hls",
            "-hls_init_time",
            "4",
            "-hls_time",
            "4",
            "-hls_list_size",
            "5",
            "-hls_flags",
            "temp_file+delete_segments+independent_segments",
            "-strftime",
            "1",
            "-strftime_mkdir",
            "1"
            PATH_480_30,
        ])
        .status()
        .await;
}