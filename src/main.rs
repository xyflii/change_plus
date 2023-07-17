use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::{env, fs, fs::File};
use walkdir::WalkDir;

fn main() {
    println!("-----------------------------------------");
    let path_curr = env::current_dir().unwrap();
    println!("当前路径为:{:?}", path_curr);
    let mut path = String::new();
    let mut quit = String::new();
    let mut word_from = String::new();
    let mut word_to = String::new();
    loop {
        println!("替换请输入yes,还原请输入no:");
        std::io::stdin().read_line(&mut path).unwrap();
        println!("输入为:{}", path);

        if path.trim().eq(&String::from("yes")) {
            word_from = "+".to_string();
            word_to = "%2B".to_string();
            break;
        } else if path.trim().eq(&String::from("no")) {
            word_from = "%2B".to_string();
            word_to = "+".to_string();
            break;
        } else {
            println!("非法输入,请重新输入!");
            path = String::new();
        }
    }
    match rewrite_file(&path_curr, &word_from, &word_to) {
        Ok(()) => (),
        Err(error) => {
            println!("文件操作失败:{:?}", error)
        }
    };
    match  recur_change_dir(&path_curr, &word_from, &word_to) {
        Ok(())=>(),
        Err(error)=>{
            println!("文件夹修改失败:{:?}",error)
        }
    }
    println!("按回车键退出");
    std::io::stdin().read_line(&mut quit).unwrap();
}

fn rewrite_file(path: &PathBuf, from: &String, to: &String) -> Result<(), io::Error> {
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
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

fn recur_change_dir(path:&PathBuf,from: &String,to: &String)->Result<(),io::Error>{
    let dir = fs::read_dir(path).unwrap();
    for entry_result in dir{
        let entry = match entry_result {
            Ok(entry)=>entry,
            Err(err)=>{
                println!("2-文件读取失败:{:?}",err);
                continue;
            }
        };
        println!("entry:{:?}",entry.path());
        if entry.path().is_dir(){
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