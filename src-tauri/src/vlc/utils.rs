use std::{
    fs::{self, DirEntry},
    panic,
    path::PathBuf,
};

use anitomy::{Element, ElementKind};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct AnimeData {
    title: String,
    episode: String,
    audio_term: Option<String>,
    device_compatibility: Option<String>,
    episode_title: Option<String>,
    episode_alt: Option<String>,
    file_checksum: Option<String>,
    file_extension: Option<String>,
    language: Option<String>,
    other: Option<String>,
    release_group: Option<String>,
    release_information: Option<String>,
    release_version: Option<String>,
    season: Option<String>,
    source: Option<String>,
    subtitles: Option<String>,
    type_: Option<String>,
    video_resolution: Option<String>,
    video_term: Option<String>,
    volume: Option<String>,
    year: Option<String>,
    date: Option<String>,
}

pub fn anime_data_to_json(anime: &Vec<Element<'_>>) -> AnimeData {
    let mut data = AnimeData {
        title: String::from(""),
        episode: String::from(""),
        audio_term: None,
        device_compatibility: None,
        episode_title: None,
        episode_alt: None,
        file_checksum: None,
        file_extension: None,
        language: None,
        other: None,
        release_group: None,
        release_information: None,
        release_version: None,
        season: None,
        source: None,
        subtitles: None,
        type_: None,
        video_resolution: None,
        video_term: None,
        volume: None,
        year: None,
        date: None,
    };

    anime.iter().for_each(|el| match el.kind() {
        ElementKind::Episode => data.episode = el.value().to_string(),
        ElementKind::AudioTerm => data.audio_term = Some(el.value().to_string()),
        ElementKind::DeviceCompatibility => {
            data.device_compatibility = Some(el.value().to_string())
        }
        ElementKind::EpisodeTitle => data.episode_title = Some(el.value().to_string()),
        ElementKind::EpisodeAlt => data.episode_alt = Some(el.value().to_string()),
        ElementKind::FileChecksum => data.file_checksum = Some(el.value().to_string()),
        ElementKind::FileExtension => data.file_extension = Some(el.value().to_string()),
        ElementKind::Language => data.language = Some(el.value().to_string()),
        ElementKind::Other => data.other = Some(el.value().to_string()),
        ElementKind::ReleaseGroup => data.release_group = Some(el.value().to_string()),
        ElementKind::ReleaseInformation => data.release_information = Some(el.value().to_string()),
        ElementKind::ReleaseVersion => data.release_version = Some(el.value().to_string()),
        ElementKind::Season => data.season = Some(el.value().to_string()),
        ElementKind::Source => data.source = Some(el.value().to_string()),
        ElementKind::Subtitles => data.subtitles = Some(el.value().to_string()),
        ElementKind::Title => data.title = el.value().to_string(),
        ElementKind::Type => data.type_ = Some(el.value().to_string()),
        ElementKind::VideoResolution => data.video_resolution = Some(el.value().to_string()),
        ElementKind::VideoTerm => data.video_term = Some(el.value().to_string()),
        ElementKind::Volume => data.volume = Some(el.value().to_string()),
        ElementKind::Year => data.year = Some(el.value().to_string()),
        ElementKind::Date => data.date = Some(el.value().to_string()),
    });

    data
}

pub fn get_anime() -> Option<AnimeData> {
    let pids = get_vlc_pids();

    if pids.len() > 0 {
        let fd_dir: String = format!("/proc/{}/fd", pids[0]);
        let mut anime_data: Option<AnimeData> = None;

        for entry in fs::read_dir(fd_dir).unwrap() {
            let entry: DirEntry = entry.unwrap();
            let path: PathBuf = entry.path();

            if let Ok(target) = fs::read_link(&path) {
                if let Some(filename_os_str) = target.file_name() {
                    let filename = filename_os_str.to_str().unwrap().to_string();

                    if filename.contains(".mkv") {
                        let anime_parsed = panic::catch_unwind(|| anitomy::parse(&filename));

                        match anime_parsed {
                            Ok(data) => {
                                anime_data = Some(anime_data_to_json(&data));
                            }
                            Err(_) => {
                                eprintln!("parsed anime {}", filename);
                            }
                        };
                    }
                }
            }
        }

        anime_data
    } else {
        println!("VLC is not running");

        None
    }
}

pub fn get_vlc_pids() -> Vec<u32> {
    let process_name = "vlc";
    let mut pids = Vec::new();

    for entry in fs::read_dir("/proc").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if let Some(pid_str) = path.file_name().and_then(|name| name.to_str()) {
            if pid_str.chars().all(char::is_numeric) {
                let comm_path = path.join("comm");

                if let Ok(name) = fs::read_to_string(&comm_path) {
                    if name.trim() == process_name {
                        if let Ok(pid) = pid_str.parse::<u32>() {
                            pids.push(pid);
                        }
                    }
                }
            }
        }
    }

    pids
}
