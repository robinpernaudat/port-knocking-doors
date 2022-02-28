//! We manage here the configuration file. 
//! 
//! The configuration file is formatted in UML.
//! The path file could be specified or at some default position.
//! 
//! If it specified in argument then we use it.
//! Else if it is in the user's home directory, we use it.
//! Else if is in global environnement, we use it.
//! Else we will work with default values.

use serde_derive::Deserialize;

pub static mut CONFIGURATION: Config= Config{ports_to_controls: vec![], opening_sequence: vec![]};

#[derive(Deserialize)]
pub struct Config{
    pub ports_to_controls: Vec<u16>,
    pub opening_sequence: Vec<u16>,
}

fn load(path: String)->Result<Config, String>{
    let buf: String = std::fs::read_to_string(path).map_err(|e| format!("Can't read the configuration file. ({})",e))?;
    let c: Config = toml::from_str(buf.as_str()).map_err(|e| format!("Can't parse the toml file : ({})", e))?;
    Ok(c)
}

pub fn load_conf()->Config{
    // try arg
    let arg: String = unsafe{crate::MAIN_ARGS.clone().unwrap().conf};
    if std::path::Path::new(arg.as_str()).exists() {
        if let Ok(c) = load(arg) {
            return c;
        }
    }

    //try home directory
    if let Some(home) = dirs::home_dir(){
        let p: String = format!("{}/.port_knocker.toml", home.to_str().unwrap());
        if std::path::Path::new(p.clone().as_str()).exists() {
            if let Ok(c2) = load(p) {
                return c2;
            }
        }
    }

    //try etc 
    let ch_etc = "/etc/port_knocker.toml";
    if std::path::Path::new(ch_etc).exists(){
        if let Ok(c3) = load(ch_etc.to_string()){
            return c3;
        }
    }

    //use default
    toml::from_str(r#"
    ports_to_controls = [22]
    opening_sequence = [16001,16002,16003,16004,16005]
    "#).unwrap()
}