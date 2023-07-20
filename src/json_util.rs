use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::fs::File;
use walkdir::WalkDir;

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

/**
 * @description: 替换指定路径下的所有json文件中的字符
 * @param {&PathBuf} path  指定的路径
 * @param {&String} from  被替换的字符
 * @param {&String} to    要替换成的字符
 * @return {Result<(), io::Error>}      Result::Ok
 */
pub fn rewrite_only_json(path: &PathBuf, from: &String, to: &String) -> Result<(), io::Error> {
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
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
                let  new_data: HashMap<String, Value> = serde_json::from_str(&data).unwrap();
                let write_data = serde_json::to_string(&rewrite_json(new_data, from, to)).unwrap();
                println!("write_data:{:#?}", write_data);
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


/**
 * @description:             确认文件后缀名
 * @param {String} str       传入的文件名
 * @param {String} target    需要确认的后缀
 * @return {bool}            true:一致, false:不一致
 */
pub fn confirm_ending(str: &str, target: &String) -> bool {
    let start = str.len() - target.len();
    let newstr = &str[start..];
    if newstr == target {
        return true;
    }
    return false;
}




/**
 * @description: 替换json中的字符
 * @param {HashMap<string,Value>} map  传入的反序列化后的json
 * @param {&String} from    被替换的字符 
 * @param {&String} to      要替换的字符
 * @return {HashMap<String,Value}   返回的替换后的serde json
 */
pub fn rewrite_json<'a>(
    mut map: HashMap<String, Value>,
    from: &String,
    to: &String,
) -> HashMap<String, Value> {
    let map2 = &mut map;
    if map2["root"]["content"].is_object() {
        let str = map2["root"]["content"]["uri"].as_str();
        if let Some(mut str) = str {
            let str2 = &str.replace(&*from, &*to);
            str = str2;
            map2.get_mut("root").unwrap()["content"]["uri"] = serde_json::to_value(str).unwrap();
            //    println!("str:{}",str);
        }
    }
    if map["root"]["children"].is_array() {
        let children = map
            .entry(String::from("root"))
            .or_insert(serde_json::to_value(String::from("{}")).unwrap())["children"]
            .as_array_mut()
            .unwrap();
        for item in children {
            if let Some(mut str3) = item["content"]["uri"].as_str() {
                let str2 = &str3.replace(&*from, &*to);
                str3 = str2;
                item["content"]["uri"] = serde_json::to_value(str3).unwrap();
                rewrite_children(item, from, to);
            }
        }
    };
    return map;
}

/**
 * @description: 替换子元素children中content里的uri
 * @param {&serde_json::Value} item   子元素
 * @param {&String} from   被替换的字符
 * @param {&String} to     要替换的字符
 * @return {*}       ()   
 */
pub fn rewrite_children(item: &mut Value, from: &String, to: &String) -> () {
    if let Some(item_vecc) = item["children"].as_array_mut() {
        for inner in item_vecc {
            if let Some(mut str) = inner["content"]["uri"].as_str() {
                let str2 = &str.replace(from, to);
                str = str2;
                inner["content"]["uri"] = serde_json::to_value(str).unwrap();
            }
            if let Some(_) = inner["children"].as_array_mut() {
                    rewrite_children(inner, from, to);
            }
        }
    }
}
