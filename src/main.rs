use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;

fn main() -> Result<()> {
    let env_map = load_env_map()?;

    let ignore_lines = load_ignore_lines(&env_map)?;

    let dst_file = Path::new(&env_map["APPDATA"]).join("Nutstore/db1/customExtRules.conf");
    std::fs::write(dst_file, ignore_lines.join("\n"))?;

    Ok(())
}

fn load_env_map() -> Result<HashMap<String, String>> {
    let mut env_map = HashMap::new();

    //读取系统环境变量
    for (key, value) in std::env::vars() {
        env_map.insert(key, value);
    }

    //读取文件环境变量

    for line in std::fs::read_to_string("env.txt")?.lines() {
        let mut split = line.splitn(2, "=");

        let Some(key) = split.next() else {
            continue;
        };
        let Some(value) = split.next() else {
            continue;
        };

        env_map.insert(key.to_string(), get_abs_env(&env_map, value)?);
    }

    Ok(env_map)
}

fn get_abs_env(map: &HashMap<String, String>, relative_env: &str) -> Result<String> {
    let re = Regex::new(r"%([^%]+)%")?;

    let mut replace_map = HashMap::new();

    for cap in re.captures_iter(relative_env) {
        let key = &cap[1];

        let Some(env) = map.get(key) else {
            continue;
        };

        replace_map.insert(key.to_string(), env.to_string());
    }

    let mut env = relative_env.to_string();

    for (key, value) in replace_map {
        env = env.replace(&format!("%{}%", key), &value);
    }

    Ok(env)
}

fn load_ignore_lines(env_map: &HashMap<String, String>) -> Result<Vec<String>> {
    let mut ignore_lines = Vec::new();

    for line in std::fs::read_to_string("ignore.txt")?.lines() {
        let line = get_abs_env(env_map, line)?;
        ignore_lines.push(line);
    }

    Ok(ignore_lines)
}
