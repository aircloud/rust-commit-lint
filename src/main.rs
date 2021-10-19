mod test;

#[macro_use]
extern crate clap;
use clap::App;

use std::fs;
use regex::Regex;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct LintConfig {
    commit_types: Vec<String>,
    commit_scopes: Vec<String>,
}

pub(crate) fn get_lint_config(config_path: &str) -> LintConfig {
    let contents = fs::read_to_string(config_path)
        .expect("Something went wrong reading the file");
    
    let config: LintConfig = toml::from_str(contents.as_str()).unwrap();

    return config;
}

/// 根据配置路径和信息测试是否通过
pub fn judge_message_lint_pass(config_path: &str, message: &str) -> bool {

    let skip_regex = Regex::new(r"^Merge branch").unwrap();

    if skip_regex.is_match(message) {
        println!("skip this messge check");
        return true;
    }

    let lint_config = get_lint_config(config_path);

    let reg_source = format!(r"^({})\((({}),?)*\):.{{4,}}?", lint_config.commit_types.join("|"), lint_config.commit_scopes.join("|"));
    let commit_regex = Regex::new(reg_source.as_str()).unwrap();

    // println!("commit_regex: {:?}", commit_regex);
    return commit_regex.is_match(message);
}

pub fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let pass = judge_message_lint_pass(matches.value_of("config_path").unwrap(), matches.value_of("message").unwrap());

    if !pass {
        eprintln!("commit-lint error: 提交信息不规范，请重新检查( TYPE 和 SCOPE 请参考 .toml 配置文件)");
        println!("提交规范: TYPE(SCOPE): MESSAGE");
        std::process::exit(-1);
    }

    println!("commit-lint pass");
}