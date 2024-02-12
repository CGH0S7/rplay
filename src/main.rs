use hound;
use rodio::{Decoder, OutputStream, Sink};
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 检查命令行参数数量
    if args.len() != 2 {
        println!("Usage: rplay <audio_file>");
        return;
    }

    // 提取文件名参数
    let file_name = &args[1];

    // 打印音频文件信息
    match print_audio_file_info(file_name) {
        Ok(_) => {}
        Err(e) => {
            println!("Failed to print audio file info: {}", e);
            return;
        }
    }

    // 创建音频输出流
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // 加载音频文件
    let file = match File::open(file_name) {
        Ok(f) => f,
        Err(_) => {
            println!("Failed to open file: {}", file_name);
            return;
        }
    };
    let source = match Decoder::new(BufReader::new(file)) {
        Ok(s) => s,
        Err(_) => {
            println!("Failed to decode audio file: {}", file_name);
            return;
        }
    };

    // 将音频添加到音频播放队列中
    sink.append(source);

    // 播放音频
    sink.play();

    // 等待音频播放完成并退出
    while sink.empty() == false {
        std::thread::sleep(Duration::from_millis(100));
    }

    // 等待一小段时间以确保音频播放完全结束
    std::thread::sleep(Duration::from_secs(1));
    println!("Done.");
}

fn print_audio_file_info(file_name: &str) -> Result<(), String> {
    let reader = match hound::WavReader::open(file_name) {
        Ok(r) => r,
        Err(e) => return Err(format!("Failed to open file: {}", e)),
    };

    println!("\n{}:\n", file_name);
    println!(
        " File Size: {:.2}M\n  Bit Rate: {:.2}M",
        reader.duration() as f64 * reader.spec().bits_per_sample as f64 / 4.0 / 1024.0 / 1024.0,
        reader.spec().bits_per_sample as f64 * reader.spec().sample_rate as f64 / 1024.0 / 1024.0
    );
    let encoding = match reader.spec().sample_format {
        hound::SampleFormat::Int => "Integer PCM",
        hound::SampleFormat::Float => "Floating Point PCM",
    };
    println!("  Encoding: {}", encoding);
    println!(
        "  Channels: {} @ {}-bit",
        reader.spec().channels,
        reader.spec().bits_per_sample
    );
    println!("Samplerate: {}Hz", reader.spec().sample_rate);
    let duration_seconds = reader.duration() as f64 / reader.spec().sample_rate as f64;
    let hours = duration_seconds as u32 / 3600;
    let minutes = (duration_seconds as u32 % 3600) / 60;
    let seconds = duration_seconds as u32 % 60;
    let milliseconds = ((duration_seconds - duration_seconds.floor()) * 100.0) as u32;
    println!(
        "  Duration: {:02}:{:02}:{:02}.{:02}",
        hours, minutes, seconds, milliseconds
    );

    Ok(())
}
