use std::{fs::{File, OpenOptions}, path::Path, io::Write};

use serde_yaml;
use serde::{Serialize, Deserialize};
use crate::util::error::SuperError;

const FILE_NAME: &str = "playlist.yml";

#[derive(Serialize, Deserialize)]
pub struct PlayList{
    directories: Vec<Directory>,
    selected_item: String,
}

#[derive(Serialize, Deserialize)]
pub struct Directory{
    name: String,
    path: String,    
    items: Vec<PlayListItem>,
}

#[derive(Serialize, Deserialize)]
pub struct PlayListItem{
    title: String,
    path: String,
    duration: String,
}

/// Load the playlist from disk
pub fn load() -> Result<PlayList, SuperError>{
    // If the file does not exist, create it
    if !Path::new(FILE_NAME).try_exists()?{
        File::create(FILE_NAME)?;
    }

    let f = File::open(FILE_NAME)?;
    let play_list = serde_yaml::from_reader(f)?;

    Ok(play_list)
}

/// Save the playlist to disk
pub fn save(play_list: PlayList) -> Result<(), SuperError>{
    // If the file does not exist, create it
    if !Path::new(FILE_NAME).try_exists()?{
        File::create(FILE_NAME)?;
    }

    let mut f = OpenOptions::new()
                    .write(true)
                    .append(false)
                    .open(FILE_NAME)?;
    serde_yaml::to_writer(f, &play_list)?;
    f.flush()?;

    Ok(())
}