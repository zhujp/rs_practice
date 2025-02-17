use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::{WavSpec, WavWriter,WavReader};
use rustfft::{FftPlanner, num_complex::Complex};
// use std::f32::consts::PI;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化 CPAL 主机
    let delay_time:u64 = 10;
    let _ = record_audio("output.wav".to_string(),delay_time);
    let _ = anylize_wav("output.wav".to_string());
    Ok(())
}

fn anylize_wav(audio_path:String) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = WavReader::open(audio_path)?;
    let samples: Vec<f32> = reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32 / i16::MAX as f32) // 归一化到 [-1.0, 1.0]
        .collect();

    // 准备 FFT
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(samples.len());

    // 将音频数据转换为复数格式
    let mut buffer: Vec<Complex<f32>> = samples
        .into_iter()
        .map(|x| Complex::new(x, 0.0)) // 实部为音频数据，虚部为 0
        .collect();
    // 执行 FFT
    fft.process(&mut buffer);

    // 计算频谱（取模）
    let spectrum: Vec<f32> = buffer
        .iter()
        .map(|c| c.norm()) // 计算复数的模
        .collect();

    // 输出频谱数据到文件
    let mut file = File::create("spectrum.txt")?;
    for (i, value) in spectrum.iter().enumerate() {
        writeln!(file, "{}: {}", i, value)?;
    }

    println!("频谱分析完成，结果已保存到 spectrum.txt");

    Ok(())
}

fn record_audio(audio_path:String,delay_time:u64) -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();

    // 获取默认输入设备（通常是系统音频输出）
    let device = host
        .default_input_device()
        .expect("Failed to get default input device");

    // 获取默认输入格式
    let supported_format = device
        .default_input_config()
        .expect("Failed to get default input format");

    // 从 SupportedStreamConfig 中提取 StreamConfig
    let config = supported_format.config();

    println!("Selected device: {}", device.name()?);
    println!("Selected format: {:?}", config);

    // 创建 WAV 文件
    let spec = WavSpec {
        channels: config.channels as u16,
        sample_rate: config.sample_rate.0 as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let writer = Arc::new(Mutex::new(WavWriter::create(audio_path, spec)?));

    // 音频数据回调函数
    let writer_clone = writer.clone();
    let stream = device.build_input_stream(
        &config, // 使用 StreamConfig
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut writer = writer_clone.lock().unwrap();
            for sample in data {
                let sample_i16 = (sample * i16::MAX as f32) as i16;
                writer.write_sample(sample_i16).unwrap();
            }
        },
        move |err| {
            eprintln!("An error occurred on the input audio stream: {}", err);
        },
        None, // 不使用超时
    )?;

    // 开始录制
    stream.play()?;
    println!("Recording started...");

    // 录制 5 秒
    std::thread::sleep(Duration::from_secs(delay_time));

    // 停止录制
    drop(stream);

    // 显式释放 WavWriter 的所有权并调用 finalize
    let  writer = Arc::try_unwrap(writer)
        .map_err(|_| "Failed to unwrap Arc")?
        .into_inner()
        .map_err(|_| "Failed to unlock Mutex")?;
    writer.finalize()?;
    println!("Recording finished. Audio saved to output.wav");

    Ok(())
}