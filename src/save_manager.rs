use std::{fs, env};
use std::time::SystemTime;
use rand::{Rng, thread_rng};
use std::io::Write;
use serde::{Serialize, Deserialize};

use crate::map;

pub struct SaveManager {
    pub time_of_last_save: SystemTime,
}


// this is perhaps the laziest code ive ever written, i should clean this up before making this public
impl SaveManager {

    pub fn new() -> Self {
        SaveManager { time_of_last_save: SystemTime::now() }
    }

    pub fn save_map(&mut self, map: &mut map::Map) -> std::io::Result <()> {

        let now = SystemTime::now();
        let time_since_last_save = now.duration_since(self.time_of_last_save).unwrap();

        if time_since_last_save.as_secs() < 2 {
            //println!("saved less than 2 seconds ago"); 
            return Ok(());
        }
        self.time_of_last_save = now;

        let mut cwd = env::current_dir().unwrap();
        cwd.push("maps");
        let result = fs::create_dir(&cwd);

        if let Err(_) = result {
            println!("folder already exists or lacking permissions"); // figure out which
            //return result;
        }
        println!("{:?}", result);

        let mut rng = thread_rng();
        let mut valid = false;

        // this is terrible but im tired, loop until unique id is found
        while valid == false {
            valid = true;
            // 4 digit id
            let random_id = format!("map_{:?}", rng.gen_range(0..=9999));
            let target_path = cwd.join(&random_id);
            let existing_files = fs::read_dir(&cwd)?;

            for entry in existing_files {
                if format!("{:?}", entry) == random_id { valid = false; }
            }

            if valid {
                let mut file = fs::File::create(target_path)?;
                let serialized = serde_json::to_string(&map).unwrap();
                file.write_all(serialized);
                // file.write_all(format!("dimensions: {:?}\n", map.dimensions).as_bytes());
                // file.write_all(format!("tile_size: {:?}\n", map.tile_size).as_bytes());
                // file.write_all(format!("tile_states: {:?}\n", map.tile_states).as_bytes());
            }

        }

        Ok(())
    }

}

pub fn read_map(file: &str) -> map::Map {
    todo!();
}

// data serialization or something like that is likely the obvious way to go here but
// that isnt my focus in this project, saving files is just a utility and i dont want to spend
// more time than i need to polishing this

// actually serde would make this way easier than writing everything as a string lmaoooo