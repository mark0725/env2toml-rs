use std::env;

pub fn env2toml(prefix:&str) -> Result<String, String> {
    let mut var_list:Vec<(Option<String>, String, String)> = vec![];
    let mut sections:Vec<String> = vec![];

    for (k,v) in env::vars() {
        if k.starts_with(prefix) {
            let s = String::from(&k[prefix.len()..]).to_lowercase();
            let l = s.split("__");
            let keys: Vec<&str> = l.collect();
            let mut section = String::new();
            for i in 0..(keys.len()-1) {
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

            if section.len()>0 {
                var_list.push((Some(section), new_key.to_string(), v));
            } else {
                var_list.push((None, new_key.to_string(), v));
            }
            
        }
    }

    
    let mut result = String::new();
    //global vars
    for (section, k, v) in &var_list {
        if let None = section {
            result = result + &format!("{}={} \n", k, v);
        }
    }

    //section vars
    for sec in sections {
        result = result + &format!("\n[{}]\n", sec);
        for (section, k, v) in &var_list {
            if let Some(s) = section {
                if sec == *s {
                    result = result + &format!("{}={} \n", k, v);
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