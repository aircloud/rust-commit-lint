#![feature(extended_key_value_attributes)]
#![feature(osstring_ascii)]

#[macro_use]
extern crate clap;
use clap::App;

use std::fs;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    commit_types: Vec<String>,
    commit_scopes: Vec<String>,
}

// argv 解析
// toml 解析
use std::env;

pub fn main() {
        // The YAML file is found relative to the current file, similar to how modules are found
        let yaml = load_yaml!("cli.yml");
        let matches = App::from_yaml(yaml).get_matches();
    
        // Same as previous examples...


    println!("opt.message: {:?}", matches.value_of("message"));
    println!("opt.config_path: {:?}", matches.value_of("config_path"));

    let contents = fs::read_to_string(matches.value_of("config_path").unwrap())
        .expect("Something went wrong reading the file");
    
    let config: Config = toml::from_str(contents.as_str()).unwrap();
    println!("config: {:?}", config);

    use regex::Regex;

    let reg_source = format!(r"^({})\(({}),?*\):.{{4,}}?$", config.commit_types.join("|"), config.commit_scopes.join("|"));
    println!("reg_source: {}", reg_source);
    let re = Regex::new(reg_source.as_str()).unwrap();

    println!("re: {}", re);

    println!("1: {}", re.is_match("feat(all,common):1"));
    println!("2: {}", re.is_match("feat(all):123"));
    println!("3: {}", re.is_match("chore(common):12345"));
    println!("4: {}", re.is_match("choreommon):123"));

    // more program logic goes here...
}