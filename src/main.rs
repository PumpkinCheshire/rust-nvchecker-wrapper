use serde_json::from_str;
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;

struct Pair {
    name: String,
    old: String,
    new: String,
}

fn read_from_file<P: AsRef<Path>>(path: P) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let data = read_to_string(path).expect("Something went wrong reading the file");

    let json: HashMap<String, String> = from_str(&data)?;

    Ok(json)
}

fn map_json_to_vec(json: HashMap<String, String>) -> Vec<Pair> {
    let mut pairs: Vec<Pair> = Vec::new();
    for (name, version) in json.iter() {
        if name.starts_with("aur-") {
            let true_name = if name.ends_with("-bin") {
                &name[4..&name.len() - 4]
            } else {
                &name[4..]
            };

            let mut github = "github-".to_string();
            github.push_str(&true_name);

            // let mut pypi = "python-".to_string();
            // pypi.push_str(&true_name);

            let pair = Pair {
                name: true_name.to_string(),
                old: version.to_string(),
                new: if json.contains_key(&github) {
                    match json.get(&github) {
                        Some(new_version) => {
                            if new_version.starts_with("v") {
                                new_version[1..].to_string()
                            } else {
                                new_version.to_string()
                            }
                        }
                        None => "non".to_string(),
                    }
                } else {
                    match json.get(&true_name.to_string()) {
                        Some(new_version) => {
                            if new_version.starts_with("v") {
                                new_version[1..].to_string()
                            } else {
                                new_version.to_string()
                            }
                        }
                        None => "non".to_string(),
                    }
                },
            };
            pairs.push(pair);
        }
    }
    pairs
}

fn check_new(pair: &Pair) {
    match &pair.new == &pair.old {
        true => println!("{} up to date.", &pair.name),
        _ => println!(
            "Please bump {} from {} to {}.",
            &pair.name, &pair.old, &pair.new
        ),
    }
}

fn main() {
    let json = read_from_file("new.json").unwrap();
    let pairs = map_json_to_vec(json);
    for pair in pairs.iter() {
        check_new(&pair);
    }
}
