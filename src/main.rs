use std::{fs::{self, File}, default, io::Write};

use serde_yaml::{Mapping, Value};

fn main() {
    let definition_conent = fs::read_to_string("/Users/mkersten/Repositories/oas-overlay/openapi.yml").unwrap();
    let overlay_content = fs::read_to_string("/Users/mkersten/Repositories/oas-overlay/overlay.yml").unwrap();
    let deserialized_defintion: Mapping = serde_yaml::from_str(&definition_conent).unwrap();
    let deserialized_overlay: Mapping = serde_yaml::from_str(&overlay_content).unwrap();
    let result = process_overlay(deserialized_defintion, deserialized_overlay);
    let serialized = serde_yaml::to_string(&result).unwrap();
    let mut file = File::create("/Users/mkersten/Repositories/oas-overlay/result.yml").unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
}

fn process_overlay(definition: Mapping, overlay: Mapping) -> Mapping {
    let mut result = definition.clone();
    for(key, value) in overlay {
        if let Some(definition_value) = result.get_mut(&key) {
            if definition_value.is_mapping() {
                println!("mapping {:?}", key);
                let def = definition_value.clone();
                *definition_value = Value::from(process_overlay(def.as_mapping().unwrap().clone(), value.as_mapping().unwrap().clone()));
            } else {
                println!("value: {:?} {:?}", key, value);
                *result.get_mut(&key).unwrap() = value
            }
        } else {
            result.insert(key, value);
        }
    }
    result
}
