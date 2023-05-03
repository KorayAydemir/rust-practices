use std::{
    fs::{self, File},
    io::{self, stdout, Result, Write},
};

#[cfg(test)]
mod tests {
    #[test]
    fn saving_pass() {
        use crate::read_saved_pass;
        use crate::save_pass;

        save_pass("testpass");
        let saved_pass = read_saved_pass().unwrap();

        assert_eq!(saved_pass, "testpass");
    }

    #[test]
    fn check_pass() {
        use crate::is_pass_correct;
        use crate::Config;

        let correct_pass = Config {
            saved_pass: "koray_123!*".to_string(),
            entered_pass: "koray_123!*".to_string(),
        };

        let incorrect_pass = Config {
            saved_pass: "koray_123!*".to_string(),
            entered_pass: "bambambam".to_string(),
        };

        assert_eq!(is_pass_correct(correct_pass), true);
        assert_ne!(is_pass_correct(incorrect_pass), true);
    }
}

pub struct Config {
    saved_pass: String,
    pub entered_pass: String,
}

enum ConfigKind {
    Save,
    DontSave,
}

pub fn run() -> Result<()> {
    if read_saved_pass().is_err() || read_saved_pass().unwrap() == "" {
        println!("You don't have a saved pass enter a new one");
        Config::build(ConfigKind::Save)?;
        Ok(())
    } else {
        let config = Config::build(ConfigKind::DontSave)?;
        if is_pass_correct(config) {
            println!("you have gained access");
            Ok(())
        } else {
            println!("wrong password, exiting program");
            std::process::exit(0);
        }
    }
}

impl Config {
    fn build(is_save: ConfigKind) -> Result<Config> {
        let entered_pass = enter_pass();
        let saved_pass = match is_save {
            ConfigKind::Save => {
                save_pass(&entered_pass);
                entered_pass.clone()
            }
            ConfigKind::DontSave => read_saved_pass()?,
        };

        Ok(Config {
            entered_pass,
            saved_pass,
        })
    }
}

fn read_saved_pass() -> Result<String> {
    fs::read_to_string("saved_pass.txt")
}
pub fn save_pass(entered_pass: &str) -> &str {
    let mut file = File::create("saved_pass.txt").expect("could not create the save file");
    file.write_all(entered_pass.as_bytes())
        .expect("could not write to save file");
    "your password has been saved !"
}

pub fn enter_pass() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Can't get input");
    let _ = stdout().flush();
    input
}

pub fn is_pass_correct(config: Config) -> bool {
    if config.entered_pass == config.saved_pass {
        return true;
    } else {
        return false;
    }
}
