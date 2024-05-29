use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct SongLoader  {
    song_path: String
}

enum InitError {
    InitValue,
}

impl SongLoader {

    pub fn new(path: String) -> Result<Self, InitError>{
        if path.ends_with(".ach") {
            Ok(
                SongLoader {
                    song_path: path,
                }
            )
        }
        else {
            Err(InitError::InitValue)
        }
    }


    pub fn read_file(&self) -> Result<(), InitError> {
        let file = File::open(self.song_path);

        let buffer = BufReader::new(file);

        for line in buffer.lines() { 
        }

        Ok(())
    }

}
