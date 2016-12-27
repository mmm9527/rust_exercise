use std::process::*;
use std::env::args;
use std::io;
use std::fs::{self, File, DirEntry};
use std::path::Path;


fn visit_dirs(dir: &Path, pattern: &String, cb: &Fn(&DirEntry, &String)) -> io::Result<()> {
    if fs::metadata(dir)?.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            if fs::metadata(entry.path())?.is_dir() {
                visit_dirs(&entry.path(), pattern, cb)?;
            } else {
                cb(&entry, pattern);
            }
        }
    } else {
        let entry = fs::read_dir(dir)?.next().unwrap()?;
        cb(&entry, pattern);
    }
    Ok(())
}

fn call_back(de: &DirEntry, pt: &String) {
    let mut f = File::open(de.path()).unwrap();
    let mut buf = io::BufReader::new(f);
    for line in io::BufRead::lines(buf) {
        let line = line.unwrap_or("".to_string());
        if line.contains(pt) {
            println!("{}", &line);
        }
    }
}


pub fn std_test() {
    let mut arg_iter = args();
    arg_iter.next().unwrap();
    // panic if there is no one
    arg_iter.next().unwrap();
    let pattern = arg_iter.next().unwrap_or("main".to_string());
    let pt = arg_iter.next().unwrap_or("./".to_string());
    let output = Command::new("/usr/bin/grep")
        .arg("-n")
        .arg("-r")
        .arg("--color")
        .arg(&pattern)
        .arg(&pt)
        .output()
        .unwrap_or_else(|e| panic!("wg panic because:{}", e));
    println!("output:");
    let st = String::from_utf8_lossy(&output.stdout);
    let lines = st.split("\n");
    for line in lines {
        println!("{}", line);
    }
    println!("===================子进程===================");
    //子进程
    let child = Command::new("/usr/bin/grep")
        .arg("-n")
        .arg("-r")
        .arg(&pattern)
        .arg(&pt)
        // 设置pipeline
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    // 做些其他的事情
    ::std::thread::sleep_ms(1000);
    println!("{}", "计算很费时间……");
    let out = child.wait_with_output().unwrap();
    let out_str = String::from_utf8_lossy(&out.stdout);
    for line in out_str.split("\n") {
        println!("{}", line);
    }
    // 实现调用grep命令搜索文件
    let mut arg_iter = args();
    arg_iter.next();
    arg_iter.next();
    // panic if there is no one
    let pattern = arg_iter.next().unwrap_or("main".to_string());
    let pt = arg_iter.next().unwrap_or("./".to_string());
    println!("pattern={},dir={}", pattern, pt);
    let pt = Path::new(&pt);
    visit_dirs(&pt, &pattern, &call_back).unwrap();
}