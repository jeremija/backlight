use std::default::Default;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::io;
use std::io::Write;
use std::path::{PathBuf};
use std::process::exit;
use std::vec::Vec;

struct Brightness {
    backend: String,
    max_brightness: i32,
}

impl std::default::Default for Brightness {
    fn default() -> Brightness {
        return Brightness {
            backend: "intel_backlight".to_string(),
            max_brightness: 0,
        }
    }
}

impl Brightness {
    fn get(&self, filename: &str) -> Result<i32, io::Error> {
        let mut path_buffer = PathBuf::from("/sys/class/backlight");
        path_buffer.push(self.backend.clone());
        path_buffer.push(filename);

        let path = path_buffer.as_path();
        let mut file = try!(File::open(path));

        let mut content = String::new();
        try!(file.read_to_string(&mut content));

        match content.trim().parse::<i32>() {
            Ok(value) => Ok(value),
            Err(err) => {
                printerr!("Error parsing value from file '{}': {}",
                         path.display(), err);
                Ok(0)
            }
        }
    }

    fn set_brightness(&self, mut value: i32) -> Result<bool, io::Error> {
        let max = try!(self.get_max_brightness());
        if value > max {
            value = max;
        } else if value < 0 {
            value = 0;
        }

        let mut path_buffer = PathBuf::from("/sys/class/backlight");
        path_buffer.push(self.backend.clone());
        path_buffer.push("brightness");

        let path = path_buffer.as_path();

        let mut file = try!(OpenOptions::new().write(true).open(path));

        match file.write_all(value.to_string().as_bytes()) {
            Ok(_) => Ok(true),
            Err(err) => Err(err)
        }
    }

    fn get_max_brightness(&self) -> Result<i32, io::Error> {
        if self.max_brightness > 0 {
            return Ok(self.max_brightness);
        }
        return self.get("max_brightness");
    }

    fn get_brightness(&self) -> Result<i32, io::Error> {
        return self.get("brightness");
    }

    fn get_percent(&self) -> Result<i32, io::Error> {
        let value = try!(self.get_brightness()) as f32;
        let max = try!(self.get_max_brightness()) as f32;
        let result = (100 as f32) * (value + 0.5) / max;
        return Ok(result as i32);
    }

    fn set_percent(&self, value: i32) -> Result<bool, io::Error> {
        let max = try!(self.get_max_brightness());
        let value = (value as f32) / (100_f32) * (max as f32) + 0.5_f32;
        let value = value as i32;
        return self.set_brightness(value as i32);
    }

}
