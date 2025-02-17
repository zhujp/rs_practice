use device_query::{DeviceQuery, DeviceState, Keycode};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::{env, thread, time::Duration};

fn main() {
    let device_state = DeviceState::new();

    println!("Press any key (Esc to exit)...");
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please provide the path to the audio file as an argument");
        return;
    }

    let audio_path = &args[1];
    let audio_data = load_audiojson(audio_path.to_string());

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        for key in &keys {
            let key_code = get_keymap_code(key.to_string());
            for (key_name, audio_file) in audio_data.iter() {
                if key_code != "" && key_code == *key_name {
                    play_audio(audio_file.to_string());
                }
            }
        }

        if keys.contains(&Keycode::Escape) {
            println!("Esc pressed, exiting...");
            break;
        }

        thread::sleep(Duration::from_millis(100));
    }
}

fn get_keymap_code(key: String) -> String {
    let mut keymap = HashMap::new();
    keymap.insert("E".to_string(), "20".to_string());
    keymap.insert("A".to_string(), "9".to_string());
    keymap.insert("T".to_string(), "32".to_string());

    // keymap.get(&key).unwrap().to_string() // 使用 .unwrap() 可能会panic,当没匹配到值得时候，会panic
    keymap.get(&key).cloned().unwrap_or_else(|| "".to_string())
}

fn load_audiojson(filepath: String) -> HashMap<String, String> {
    let file = File::open(filepath).expect("Failed to open file");
    let reader = BufReader::new(file);
    let audio_json: HashMap<String, String> =
        serde_json::from_reader(reader).expect("Failed to parse JSON");
    audio_json
}

fn play_audio(filepath: String) {
    let (_stream_handle, stream) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream).unwrap();
    // let file = File::open(filepath).expect("Failed to open file");  //当文件不存在会报错
    // let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    // sink.append(source);
    match File::open(filepath) {
        Ok(file) => {
            let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
            sink.append(source);
        }
        Err(_) => {} // 如果文件不存在或打开失败，不执行任何操作
    }

    // sink.sleep_until_end(); // 等待音频播放完毕
}
