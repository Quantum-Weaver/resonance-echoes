mod audio;
mod equalizer;
mod visualizer;

use audio::{AudioState, Cmd};
use equalizer::{EqStateResponse, preset_bands};
use base64::prelude::*;
use lofty::file::AudioFile;
use lofty::file::TaggedFileExt;
use lofty::tag::Accessor;
use lofty::tag::ItemKey;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tauri::{Emitter, Manager};
use tauri_plugin_sql::{Migration, MigrationKind};

// ── TrackInfo (returned by scan_directory, mirrored in TS) ───────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrackInfo {
    pub id: String,
    pub uri: String,
    pub filename: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub genre: Option<String>,
    pub year: Option<u32>,
    #[serde(rename = "trackNumber")]
    pub track_number: Option<u32>,
    pub duration: f64,
    #[serde(rename = "coverArt")]
    pub cover_art: Option<String>,
    pub lyrics: Option<String>,
    #[serde(rename = "dateAdded")]
    pub date_added: u64,
}

// ── Metadata parsing ──────────────────────────────────────────────────────────

fn parse_metadata(path: &Path) -> TrackInfo {
    let filename = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("Unknown")
        .to_string();

    let uri = path.to_string_lossy().to_string();
    let id = uri.clone();

    let mut title = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Unknown")
        .to_string();

    let mut artist = String::from("Unknown Artist");
    let mut album = String::from("Unknown Album");
    let mut genre: Option<String> = None;
    let mut year: Option<u32> = None;
    let mut track_number: Option<u32> = None;
    let mut cover_art: Option<String> = None;
    let mut lyrics: Option<String> = None;
    let mut duration = 0.0f64;

    if let Ok(tagged_file) = lofty::read_from_path(path) {
        duration = tagged_file.properties().duration().as_secs_f64();

        if let Some(tag) = tagged_file.primary_tag() {
            if let Some(t) = tag.title() {
                title = t.to_string();
            }
            if let Some(a) = tag.artist() {
                artist = a.to_string();
            }
            if let Some(a) = tag.album() {
                album = a.to_string();
            }
            genre = tag.genre().map(|g| g.to_string());

            year = tag
                .get_string(&ItemKey::RecordingDate)
                .and_then(|s| s.parse::<u32>().ok());
            track_number = tag
                .get_string(&ItemKey::TrackNumber)
                .and_then(|s| s.parse::<u32>().ok());
            lyrics = tag.get_string(&ItemKey::Lyrics).map(|s| s.to_string());

            if let Some(pic) = tag.pictures().first() {
                let mime = match pic.mime_type() {
                    Some(lofty::picture::MimeType::Jpeg) => "image/jpeg",
                    Some(lofty::picture::MimeType::Png) => "image/png",
                    Some(lofty::picture::MimeType::Gif) => "image/gif",
                    Some(lofty::picture::MimeType::Tiff) => "image/tiff",
                    Some(lofty::picture::MimeType::Bmp) => "image/bmp",
                    _ => "image/jpeg",
                };
                let encoded = BASE64_STANDARD.encode(pic.data());
                cover_art = Some(format!("data:{};base64,{}", mime, encoded));
            }
        }
    }

    // Fallback: "Artist - Title" filename heuristic
    if artist == "Unknown Artist" && title.contains(" - ") {
        let parts: Vec<&str> = title.splitn(2, " - ").collect();
        if parts.len() == 2 {
            artist = parts[0].trim().to_string();
            title = parts[1].trim().to_string();
        }
    }

    let date_added = fs::metadata(path)
        .and_then(|m| m.modified())
        .map(|t| {
            t.duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        })
        .unwrap_or(0);

    TrackInfo {
        id,
        uri,
        filename,
        title,
        artist,
        album,
        genre,
        year,
        track_number,
        duration,
        cover_art,
        lyrics,
        date_added,
    }
}

// ── scan_directory command ────────────────────────────────────────────────────

#[derive(Serialize, Clone)]
struct ScanProgress {
    current: usize,
    total: usize,
}

#[tauri::command]
fn scan_directory(
    app_handle: tauri::AppHandle,
    dir_path: String,
) -> Result<Vec<TrackInfo>, String> {
    let path = Path::new(&dir_path);
    if !path.exists() {
        return Err(format!("Directory not found: {dir_path}"));
    }

    // Probe storage access early so the frontend can show a useful error
    if let Err(e) = fs::read_dir(path) {
        return Err(if e.kind() == std::io::ErrorKind::PermissionDenied {
            "PERMISSION_DENIED: Storage access is required to scan music files. \
             Please grant Media or Files access in \
             Settings → Apps → Resonance Compass → Permissions."
                .to_string()
        } else {
            format!("Cannot read directory: {e}")
        });
    }

    const EXTENSIONS: &[&str] = &["mp3", "flac", "wav", "aac", "ogg", "m4a"];

    fn collect_paths(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    collect_paths(&p, out);
                } else if p
                    .extension()
                    .map(|e| EXTENSIONS.contains(&e.to_string_lossy().to_lowercase().as_str()))
                    .unwrap_or(false)
                {
                    out.push(p);
                }
            }
        }
    }

    let mut paths = Vec::new();
    collect_paths(path, &mut paths);
    let total = paths.len();

    let mut tracks = Vec::with_capacity(total);
    for (i, file_path) in paths.iter().enumerate() {
        tracks.push(parse_metadata(file_path));
        let _ = app_handle.emit("scan-progress", ScanProgress { current: i + 1, total });
    }

    Ok(tracks)
}

// ── Playback commands (delegated to audio module) ─────────────────────────────

#[tauri::command]
fn play_track(path: String, state: tauri::State<AudioState>) {
    state.send(Cmd::Play(path));
}

#[tauri::command]
fn pause(state: tauri::State<AudioState>) {
    state.send(Cmd::Pause);
}

#[tauri::command]
fn resume(state: tauri::State<AudioState>) {
    state.send(Cmd::Resume);
}

#[tauri::command]
fn stop(state: tauri::State<AudioState>) {
    state.send(Cmd::Stop);
}

#[tauri::command]
fn seek(position_secs: f64, state: tauri::State<AudioState>) {
    state.send(Cmd::Seek(position_secs));
}

#[tauri::command]
fn set_volume(vol: f32, state: tauri::State<AudioState>) {
    state.send(Cmd::Volume(vol));
}

// ── Equalizer commands ────────────────────────────────────────────────────────

#[tauri::command]
fn get_eq_state(state: tauri::State<AudioState>) -> Result<EqStateResponse, String> {
    let eq = state.eq.lock().map_err(|e| e.to_string())?;
    Ok(EqStateResponse::from_state(&eq))
}

#[tauri::command]
fn set_eq_band(state: tauri::State<AudioState>, band: usize, gain_db: f32) -> Result<(), String> {
    if band >= 10 {
        return Err(format!("band index {band} out of range 0–9"));
    }
    let mut eq = state.eq.lock().map_err(|e| e.to_string())?;
    eq.bands[band] = gain_db.clamp(-12.0, 12.0);
    Ok(())
}

#[tauri::command]
fn set_eq_preamp(state: tauri::State<AudioState>, gain_db: f32) -> Result<(), String> {
    let mut eq = state.eq.lock().map_err(|e| e.to_string())?;
    eq.preamp = gain_db.clamp(-12.0, 12.0);
    Ok(())
}

#[tauri::command]
fn toggle_eq(state: tauri::State<AudioState>, enabled: bool) -> Result<(), String> {
    let mut eq = state.eq.lock().map_err(|e| e.to_string())?;
    eq.enabled = enabled;
    Ok(())
}

#[tauri::command]
fn set_eq_preset(state: tauri::State<AudioState>, preset: String) -> Result<(), String> {
    let bands = preset_bands(&preset)
        .ok_or_else(|| format!("unknown preset: {preset}"))?;
    let mut eq = state.eq.lock().map_err(|e| e.to_string())?;
    eq.bands = bands;
    Ok(())
}

// ── Metadata fetch commands ───────────────────────────────────────────────────

#[tauri::command]
async fn fetch_lyrics(artist: String, title: String) -> Result<Option<String>, String> {
    let Ok(client) = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(8))
        .build()
    else {
        return Ok(None);
    };

    let Ok(response) = client
        .get("https://lrclib.net/api/get")
        .query(&[("artist_name", &artist), ("track_name", &title)])
        .send()
        .await
    else {
        return Ok(None);
    };

    if !response.status().is_success() {
        return Ok(None);
    }

    let Ok(json) = response.json::<serde_json::Value>().await else {
        return Ok(None);
    };

    for field in &["syncedLyrics", "plainLyrics"] {
        if let Some(text) = json.get(field).and_then(|v| v.as_str()) {
            if !text.trim().is_empty() {
                return Ok(Some(text.to_string()));
            }
        }
    }

    Ok(None)
}

#[tauri::command]
async fn fetch_cover_art(artist: String, album: String) -> Result<Option<String>, String> {
    let Ok(client) = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent("ResonanceCompass/0.1.0 (resonance-compass)")
        .build()
    else {
        return Ok(None);
    };

    // Step 1: MusicBrainz release lookup
    let query = format!("artist:{} release:{}", artist, album);
    let params = [("query", query.as_str()), ("fmt", "json"), ("limit", "5")];
    let Ok(mb_resp) = client
        .get("https://musicbrainz.org/ws/2/release/")
        .query(&params)
        .send()
        .await
    else {
        return Ok(None);
    };

    if !mb_resp.status().is_success() {
        return Ok(None);
    }

    let Ok(mb_json) = mb_resp.json::<serde_json::Value>().await else {
        return Ok(None);
    };

    let mbid = match mb_json
        .get("releases")
        .and_then(|r| r.as_array())
        .and_then(|arr| arr.first())
        .and_then(|rel| rel.get("id"))
        .and_then(|id| id.as_str())
    {
        Some(id) => id.to_string(),
        None => return Ok(None),
    };

    // Step 2: Cover Art Archive — download front image as base64 data URI
    let caa_url = format!("https://coverartarchive.org/release/{}/front", mbid);
    let Ok(caa_resp) = client.get(&caa_url).send().await else {
        return Ok(None);
    };

    if !caa_resp.status().is_success() {
        return Ok(None);
    }

    let mime = caa_resp
        .headers()
        .get("content-type")
        .and_then(|ct| ct.to_str().ok())
        .map(|ct| ct.split(';').next().unwrap_or("image/jpeg").trim().to_string())
        .unwrap_or_else(|| "image/jpeg".to_string());

    let Ok(bytes) = caa_resp.bytes().await else {
        return Ok(None);
    };

    if bytes.is_empty() {
        return Ok(None);
    }

    Ok(Some(format!("data:{};base64,{}", mime, BASE64_STANDARD.encode(&bytes))))
}

// ── Fragment commands ─────────────────────────────────────────────────────────

#[tauri::command]
async fn create_fragment(
    app_handle: tauri::AppHandle,
    source_path: String,
    start_secs: f64,
    end_secs: f64,
    output_name: String,
) -> Result<String, String> {
    let source = Path::new(&source_path);
    let ext = source
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("mp3")
        .to_lowercase();

    let fragments_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("fragments");

    fs::create_dir_all(&fragments_dir)
        .map_err(|e| format!("Cannot create fragments dir: {e}"))?;

    let safe_name: String = output_name
        .chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
            c => c,
        })
        .collect();

    let output_path = fragments_dir.join(format!("{}.{}", safe_name, ext));
    let output_str = output_path.to_string_lossy().to_string();

    let result = std::process::Command::new("ffmpeg")
        .args([
            "-y",
            "-i",
            &source_path,
            "-ss",
            &format!("{:.3}", start_secs),
            "-to",
            &format!("{:.3}", end_secs),
            "-c",
            "copy",
            &output_str,
        ])
        .output();

    match result {
        Ok(out) if out.status.success() => Ok(output_str),
        Ok(out) => Err(format!(
            "ffmpeg failed: {}",
            String::from_utf8_lossy(&out.stderr)
        )),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            Err("ffmpeg_not_found".to_string())
        }
        Err(e) => Err(format!("Failed to run ffmpeg: {e}")),
    }
}

#[tauri::command]
async fn export_fragments(paths: Vec<String>, dest_dir: String) -> Result<u32, String> {
    let dest = Path::new(&dest_dir);
    if !dest.exists() {
        return Err(format!("Destination directory does not exist: {dest_dir}"));
    }
    let mut copied = 0u32;
    for path_str in &paths {
        let src = Path::new(path_str);
        if !src.exists() {
            continue;
        }
        let file_name = src.file_name().unwrap_or_default();
        let dst = dest.join(file_name);
        if fs::copy(src, &dst).is_ok() {
            copied += 1;
        }
    }
    Ok(copied)
}

// ── App entry point ───────────────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (vis_tx, vis_rx) = visualizer::make_channel();

    let migrations = vec![
        Migration {
            version: 1,
            description: "create_songs_table",
            sql: "CREATE TABLE IF NOT EXISTS songs (
                id TEXT PRIMARY KEY,
                uri TEXT NOT NULL UNIQUE,
                filename TEXT NOT NULL,
                title TEXT NOT NULL,
                artist TEXT NOT NULL,
                album TEXT NOT NULL,
                genre TEXT,
                year INTEGER,
                track_number INTEGER,
                duration REAL NOT NULL DEFAULT 0,
                cover_art TEXT,
                date_added INTEGER NOT NULL DEFAULT 0,
                last_scanned INTEGER NOT NULL DEFAULT 0
            );",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 2,
            description: "create_mood_events_table",
            sql: "CREATE TABLE IF NOT EXISTS mood_events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                track_id TEXT NOT NULL,
                emoji TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                intensity INTEGER DEFAULT 3,
                comment TEXT,
                context TEXT DEFAULT 'manual'
            );
            CREATE INDEX IF NOT EXISTS idx_mood_track ON mood_events(track_id);
            CREATE INDEX IF NOT EXISTS idx_mood_time ON mood_events(timestamp);",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 3,
            description: "create_favorites_table",
            sql: "CREATE TABLE IF NOT EXISTS favorites (
                track_id TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                PRIMARY KEY (track_id)
            );",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 4,
            description: "add_lyrics_column",
            sql: "ALTER TABLE songs ADD COLUMN lyrics TEXT;",
            kind: MigrationKind::Up,
        },
    ];

    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:songs.db", migrations)
                .build(),
        )
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {
            visualizer::start(app.handle().clone(), vis_rx);
            let audio = AudioState::init(app.handle().clone(), vis_tx);
            app.manage(audio);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            play_track,
            pause,
            resume,
            stop,
            seek,
            set_volume,
            scan_directory,
            get_eq_state,
            set_eq_band,
            set_eq_preamp,
            toggle_eq,
            set_eq_preset,
            fetch_lyrics,
            fetch_cover_art,
            create_fragment,
            export_fragments,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Resonance Compass");
}
