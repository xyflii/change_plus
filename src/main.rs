use std::env;
mod util;
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
    match util::rewrite_file(&path_curr, &word_from, &word_to) {
        Ok(()) => (),
        Err(error) => {
            println!("文件操作失败:{:?}", error)
        }
    };
    match util::recur_change_dir(&path_curr, &word_from, &word_to) {
        Ok(())=>(),
        Err(error)=>{
            println!("文件夹修改失败:{:?}",error)
        }
    }
    println!("按回车键退出");
    std::io::stdin().read_line(&mut quit).unwrap();
}
