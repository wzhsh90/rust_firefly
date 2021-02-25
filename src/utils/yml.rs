use std::fs::File;
use std::io::Read;
use serde_yaml;
use std::collections::HashMap;

pub fn read_sql(name: &str) -> HashMap<String, String> {
    let mut yml_data = String::new();
    let file_path = format!("{}{}{}", "resource/mybatis/", name, ".yml");
    let expect_msg = format!("{}{}", file_path, " not exist");
    let de_err_msg = format!("{}{}", file_path, " deserialize error");
    File::open(file_path)
        .expect(expect_msg.as_str())
        .read_to_string(&mut yml_data);
    let mut de_map: HashMap<String, String> = serde_yaml::from_str(yml_data.as_str()).expect(de_err_msg.as_str());
    //处理模板
     replace_tpl(&mut de_map);

    return de_map;
}

fn replace_tpl(map: &mut HashMap<String, String>) {
    let mut part_map: HashMap<String, String> = HashMap::new();
    for (key, value) in map.iter() {
        if value.contains("${") {
            let mut org_val = value.clone();
            for (tpl_key, tpl_val) in map.iter() {
                let replace_key = format!("{}{}{}", "${", tpl_key, "}");
                if value.contains(replace_key.as_str()) {
                    org_val = org_val.replace(replace_key.as_str(), tpl_val.as_str());
                }
            }
            part_map.insert(key.to_string(), org_val);
        }
    }
    for (key, value) in part_map {
        map.insert(key, value);
    }
}