use std::process::*;
use std::env::args;
use std::io;
use std::fs::{self, File, DirEntry};
use std::path::Path;
use std::net::{SocketAddrV4, TcpStream, UdpSocket, TcpListener, Ipv4Addr, ToSocketAddrs};
use std::thread;
use std::net::*;
use std::io::{Read, Write};
use std::time::Duration;


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

fn server<A: ToSocketAddrs>(addr: &A) -> io::Result<()> {
    // 建立一个监听程序
    let listener = TcpListener::bind(addr)?;
    // 这个程序一次只需处理一个链接就好
    for stream in listener.incoming() {
        // 通过match再次解包 stream到
        match stream {
            // 这里匹配的重点是如何将一个mut的匹配传给一个Result
            Ok(mut st) => {
                // 我们总是要求client端先发送数据
                // 准备一个超大的缓冲区
                // 当然了，在实际的生活中我们一般会采用环形缓冲来重复利用内存。
                // 这里仅作演示，是一种很低效的做法
                let mut buf: Vec<u8> = vec![0u8; 1024];
                // 通过try!方法来解包
                // try!方法的重点是需要有特定的Error类型与之配合
                let rcount = st.read(&mut buf)?;
                // 只输出缓冲区里读取到的内容
                println!("{:?}", &buf[0..rcount]);
                // 回写内容
                let wcount = st.write(&buf[0..rcount])?;
                // 以下代码实际上算是逻辑处理
                // 并非标准库的一部分了
                if rcount != wcount {
                    panic!("Not Fully Echo!, r={}, w={}", rcount, wcount);
                }
                // 清除掉已经读到的内容
                buf.clear();
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    // 关闭掉Serve端的链接
    drop(listener);
    Ok(())
}

fn client<A: ToSocketAddrs>(addr: &A) -> io::Result<()> {
    let mut buf = vec![0u8; 1024];
    loop {
        // 对比Listener，TcpStream就简单很多了
        // 本次模拟的是tcp短链接的过程，可以看作是一个典型的HTTP交互的基础IO模拟
        // 当然，这个通讯里面并没有HTTP协议 XD！
        let mut stream = TcpStream::connect(addr).unwrap();
        let msg = "WaySLOG comming!".as_bytes();
        // 避免发送数据太快而刷屏
        thread::sleep(Duration::from_millis(100));
        let rcount = stream.write(&msg)?;
        let _ = stream.read(&mut buf)?;
        println!("{:?}", &buf[0..rcount]);
        buf.clear();
    }
}


pub fn std_test() {
    let mut arg_iter = args();
    arg_iter.next().unwrap();
    // panic if there is no one
    // arg_iter.next().unwrap();
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
    thread::sleep(Duration::from_millis(1000));
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