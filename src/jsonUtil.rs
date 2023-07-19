use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::{fs, fs::File};
use walkdir::WalkDir;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub asset: Asset,
    pub geometric_error: f64,
    pub root: Root2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub generatetool: String,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root2 {
    pub bounding_volume: BoundingVolume,
    pub children: Vec<Children>,
    pub geometric_error: f64,
    pub transform: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoundingVolume {
    #[serde(rename = "box")]
    pub box_field: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Children {
    pub bounding_volume: BoundingVolume2,
    pub children: Vec<Children2>,
    pub geometric_error: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoundingVolume2 {
    #[serde(rename = "box")]
    pub box_field: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Children2 {
    pub bounding_volume: BoundingVolume3,
    pub content: Content,
    pub geometric_error: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoundingVolume3 {
    #[serde(rename = "box")]
    pub box_field: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub uri: String,
}


pub fn rewrite_only_json(path: &PathBuf, from: &String, to: &String) -> Result<(), io::Error> {
    for entry in WalkDir::new("D:\\rustLearning\\test").into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() {
            if confirm_ending(&entry.path().to_str().unwrap(), &String::from("json")) {
                let mut src = match File::open(entry.path()) {
                    Ok(file) => file,
                    Err(error) => {
                        println!("打开文件报错:{:?}", error);
                        continue;
                    }
                };
    
                let mut data = String::new();
                match src.read_to_string(&mut data) {
                    Ok(data) => data,
                    Err(error) => {
                        println!("读取文件报错:{:?}", error);
                        continue;
                    }
                };
    
                drop(src); // Close the file early
                // println!("path:{:?}",entry.path());
                let mut new_data:HashMap<String,Value> = serde_json::from_str(&data).unwrap();
                // println!("path:{:?},new_data:{:#?}",entry.path(),new_data);
                if new_data["root"]["children"].is_array() {
                    println!("children:{:#?}",new_data["root"]["children"])
                };
                let write_data =  serde_json::to_string(&rewrite_json(new_data, from, to)).unwrap();
                println!("write_data:{:#?}",write_data);
                // let new_data = data.replace(&*from, &*to);
                let mut dst = match File::create(&entry.path()) {
                    Ok(file) => file,
                    Err(error) => {
                        println!("创建临时文件报错:{:?}", error);
                        continue;
                    }
                };
                match dst.write(write_data.as_bytes()) {
                    Ok(byte) => byte,
                    Err(error) => {
                        println!("写入文件报错:{:?}", error);
                        continue;
                    }
                };
            };
        }
    }
    Ok(())
}

pub fn confirm_ending(str: &str, target: &String) -> bool {
    let start = str.len() - target.len();
    let newstr = &str[start..];
    if newstr == target {
        return true
    }
    return false
}

pub fn rewrite_json<'a>(mut map:HashMap<String,Value>,from:&String,to:&String)->HashMap<String,Value>{
    if map["root"]["content"].is_object(){
        let str = map["root"]["content"]["uri"].as_str();
        if let Some(mut str) = str {
            let str2 = &str.replace(&*from, &*to);
           str = str2;
           map.get_mut("root").unwrap()["content"]["uri"] = serde_json::to_value(str).unwrap();
           println!("str:{}",str);
        }
    }
    if map["root"]["children"].is_array() {
        println!("children:{:#?}",map["root"]);
    };
    return map;
}
