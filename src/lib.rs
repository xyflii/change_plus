use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::{fs, fs::File};
use walkdir::WalkDir;

    /**
 * @description: 递归修改非二进制文件名称以及文件内容
 * @param {*} path  当前工作路径
 * @param {*} from  当前需要替换的内容
 * @param {*} to    当前要替换为的内容
 * @return {*}      Result::Ok()
 */
pub fn rewrite_file(path: &PathBuf, from: &String, to: &String) -> Result<(), io::Error> {
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() {
            let mut src = match File::open(entry.path()) {
                Ok(file) => file,
                Err(error) => {
                    println!("打开文件报错:{:?}", error);
                    continue;
                }
            };
            let file_name = entry.path().file_name().unwrap().to_str().unwrap();
            let new_file_name = file_name.replace(&*from, &*to);
            let new_path = entry.path().with_file_name(new_file_name);
            fs::rename(entry.path(), &new_path).unwrap();
            println!("file_name:{}", file_name);
            println!("done");
            let mut data = String::new();
            match src.read_to_string(&mut data) {
                Ok(data) => data,
                Err(error) => {
                    println!("读取文件报错:{:?}", error);
                    continue;
                }
            };

            drop(src); // Close the file early
            let new_data = data.replace(&*from, &*to);
            let mut dst = match File::create(&new_path) {
                Ok(file) => file,
                Err(error) => {
                    println!("创建临时文件报错:{:?}", error);
                    continue;
                }
            };
            match dst.write(new_data.as_bytes()) {
                Ok(byte) => byte,
                Err(error) => {
                    println!("写入文件报错:{:?}", error);
                    continue;
                }
            };
        }
    }
    Ok(())
}

/**
 * @description: 递归修改文件夹名称
 * @param {*} path  当前路径
 * @param {*} from  当前需要替换的内容
 * @param {*} to    当前要替换为的内容
 * @return {*}      Result::Ok()
 */
pub fn recur_change_dir(path: &PathBuf, from: &String, to: &String) -> Result<(), io::Error> {
    let dir = fs::read_dir(path).unwrap();
    for entry_result in dir {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(err) => {
                println!("2-文件读取失败:{:?}", err);
                continue;
            }
        };
        println!("entry:{:?}", entry.path());
        if entry.path().is_dir() {
            let oldpath = entry.path();
            let file_name = oldpath.file_name().unwrap().to_str().unwrap();
            let new_file_name = file_name.replace(&*from, &*to);
            let new_path = entry.path().with_file_name(new_file_name);
            fs::rename(entry.path(), &new_path)?;
            println!("重写文件夹:{}", file_name);
            println!("done");
            recur_change_dir(&new_path, &from, &to).unwrap();
        }
    }
    Ok(())
}

