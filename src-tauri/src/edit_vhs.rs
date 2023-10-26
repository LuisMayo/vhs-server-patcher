use std::{
    fs::{File, OpenOptions},
    iter::repeat,
    path::PathBuf,
};

use file_offset::FileExt;
use steamlocate::SteamDir;

fn unable_locate_dir() -> &'static str {
    return "Couldn't locate Steam or Vhs install dir";
}

fn unable_open_file() -> &'static str {
    return "Couldn't open VHS file, make sure the game is closed and current user has write permissions";
}

fn string_to_big() -> &'static str {
    return "String was too big!";
}

fn process_vhs_file(game_dir: &PathBuf, address: &str) -> Result<(), &'static str> {
    let file_path = game_dir.join("Game/Binaries/Win64/Game-Win64-Shipping.exe");
    let mut backup_path = file_path.clone();
    backup_path.set_extension("bak");
    let _ = std::fs::copy(&file_path, &backup_path);
    let file_result = OpenOptions::new().write(true).open(&file_path);
    match file_result {
        Ok(file) => return write_file(file, address),
        Err(_) => return Err(unable_open_file()),
    }
}

fn write_file(file: File, address: &str) -> Result<(), &'static str> {
    const BUFFER_SIZE: usize = 0x80;
    let mut buffer: Vec<u8> = address
        .encode_utf16()
        .map(|item| item.to_le_bytes())
        .flatten()
        .collect();
    if buffer.len() > BUFFER_SIZE {
        return Err(string_to_big());
    } else {
        buffer.extend(repeat(0).take(BUFFER_SIZE - buffer.len()));
        file.write_offset(&buffer, 0x5382CA0)
            .expect("Unable to write on the file");
        return Ok(());
    }
}

#[tauri::command]
pub fn edit_vhs_file(address: &str) -> Result<(), &str> {
    println!("Hello, world! {}", address);
    match SteamDir::locate() {
        Some(mut steamdir) => match steamdir.app(&611360) {
            Some(app) => return process_vhs_file(&app.path, address),
            None => return Err("Couldn't locate Steam or Vhs install dir"),
        },
        None => return Err(unable_locate_dir()),
    }
}
