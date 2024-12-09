use std::env;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct VarItem {
    section: Option<String>,
    is_array: bool,
    array_index: i32,
    key: String,
    value: String,
}

pub fn env2toml(prefix:&str) -> Result<String, String> {
    let mut var_list:Vec<VarItem> = vec![];
    let mut sections:Vec<String> = vec![];

    for (k,v) in env::vars() {
        if k.starts_with(prefix) {
            let s = String::from(&k[prefix.len()..]).to_lowercase();
            let l = s.split("__");
            let keys: Vec<&str> = l.collect();
            let mut section = String::new();
            let mut is_array = false;
            let mut array_index = -1;

            for i in 0..(keys.len()-1) {
                if let Ok(index) = keys[i].parse::<i32>() {
                    is_array = true;
                    array_index = index;
                    continue;
                }
        
                if section.len() > 0 {
                    section = section+"."+keys[i];
                } else {
                    section = keys[i].to_string();
                }

                if ! sections.contains(&section) {
                    sections.push(section.clone());
                }
               
            }

            let new_key = keys[keys.len()-1];
            let mut value = v.clone();

            if (format!("a={v}")).parse::<toml::Value>().is_err() {
                let s1 = value.replace('\\', "\\\\").replace('\"', "\\\"");
                value = format!("\"{s1}\"");
            }

            var_list.push(VarItem {
                section: if section.is_empty() { None } else { Some(section) },
                is_array,
                array_index,
                key: new_key.to_string(),
                value,
            });
        }
    }

    
    let mut result = String::new();

    // is a section
    for item in var_list.iter().filter(|item| item.section.is_none()) {
        result = result + &format!("{}={}", item.key, item.value);
    }

    // Generate TOML text for sections
    for sec in sections {
        let mut array_items: HashMap<i32, Vec<VarItem>> = HashMap::new();
        let mut is_array = false;

        for item in &var_list {
            if let Some(section) = &item.section {
                if section == &sec {
                    if item.is_array {
                        is_array = true;
                        array_items.entry(item.array_index).or_insert(Vec::new()).push(item.clone());
                    } else {
                        array_items.entry(0).or_insert(Vec::new()).push(item.clone());
                    }
                }
            }
        }

        if !is_array {
            result = result + &format!("\n[{}]\n", sec);
            if let Some(items) = array_items.get(&0) {
                for item in items {
                    result = result + &format!("{}={}\n", item.key, item.value);
                }
            }
            continue;
        }

        let mut keys: Vec<i32> = array_items.keys().cloned().collect();
        keys.sort();
        for k in keys {
            if let Some(items) = array_items.get(&k) {
                result = result + &format!("\n[[{}]]\n", sec);
                for item in items {
                    result = result + &format!("{}={}\n", item.key, item.value);
                }
            }
        }
    }

    
    return Ok(result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_env2toml() {
        dotenvy::dotenv().ok();
        let result = env2toml("APP_").unwrap();
        println!("{}\n", result);

    }
}