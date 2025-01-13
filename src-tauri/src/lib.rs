use std::thread;

use tauri::Emitter;
use vlc::utils::get_anime;

pub mod vlc;

#[tauri::command]
fn my_custom_command() {
    let t = anitomy::parse("[Erai-raws] Gate - 10 [1080p][Multiple Subtitle].mkv");
    for v in t {
        println!("{} : {}", v.kind().as_str(), v.value())
    }
}

#[tauri::command]
async fn scan_anime(app_handle: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        loop {
            if let Some(files) = get_anime() {
                let handle = app_handle.emit("anime_detected", files);
                print!("{:?}", handle);

                break;
            }
            thread::sleep(std::time::Duration::from_secs(2));
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            my_custom_command,
            scan_anime
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
