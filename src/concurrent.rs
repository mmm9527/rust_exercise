use std::thread;
use std::sync::mpsc;
use std::rc::Rc;
use std::sync::{Arc, Mutex, Condvar};
use std::time::Duration;
use std::sync::atomic::{AtomicUsize, Ordering};
use rand;

#[derive(Debug)]
enum Grade {
    A,
    B,
    C,
    D,
    E,
    None
}

#[derive(Debug)]
struct Student {
    id: i32,
    score: i32,
    grade: Grade
}

#[derive(Debug)]
struct Custom(Rc<i32>);

unsafe impl Send for Custom {}

pub fn concurrent_test() {
    println!("===================concurrent test begin===================");
    //创建线程
    //第一种方式
    let thread1 = thread::spawn(|| {
        println!("i 'am a thread ");
    });

    thread1.join().unwrap();
    //第二种方式
    let thread2 = thread::Builder::new().stack_size(4 * 1024 * 1024).name("thread2".to_string()).spawn(|| {
        println!("i 'am a second thread");
    });
    thread2.unwrap().join().unwrap();

    //练习
    // let random_range = Range::new(0, 100i32);
    let mut rng = rand::thread_rng();


    for i in 0..100 {
        let mut p = Student {
            id: i + 1,
            score: 1,
            grade: Grade::None,
        };
        thread::Builder::new().name("thread[".to_string() + &i.to_string() + "]").spawn(move || {
            match p.score {
                90 ... 100 => p.grade = Grade::A,
                80 ... 90 => p.grade = Grade::B,
                70 ... 80 => p.grade = Grade::C,
                60 ... 70 => p.grade = Grade::D,
                _ => p.grade = Grade::E,
            }
            println!("graded {:?}", p);
        }).unwrap();
    };

    //消息
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();
    thread::spawn(move || {
        tx.send(1).unwrap();
    });
    // 在主线程中接收子线程发送的消息并输出
    println!("receive {}", rx.recv().unwrap());

    //实现自定义类可以被消息通道传递
    let c = Custom(Rc::new(5));

    let (tx, rx): (mpsc::Sender<Custom>, mpsc::Receiver<Custom>) = mpsc::channel();
    thread::spawn(move || {
        tx.send(c).unwrap();
    });
    println!("receive {:?}", rx.recv().unwrap());

    //异步消息
    const THREAD_COUNT: i32 = 2;

    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();

    for id in 0..THREAD_COUNT {
        let thread_tx = tx.clone();
        thread::spawn(move || {
            thread_tx.send(id + 1).unwrap();
            println!("send {}", id + 1);
        });
    }

    thread::sleep(Duration::from_millis(2000));
    println!("wake up");

    for i in 0..THREAD_COUNT {
        println!("receiver {}", rx.recv().unwrap());
    }

    //同步通道

    let (tx, rx): (mpsc::SyncSender<i32>, mpsc::Receiver<i32>) = mpsc::sync_channel(0);

    let thread_send = thread::spawn(move || {
        println!("before send");
        tx.send(1).unwrap();
        println!("after send");
    });
    println!("before sleep");
    thread::sleep(Duration::from_millis(2000));
    println!("after sleep");
    println!("receive {}", rx.recv().unwrap());
    thread_send.join().unwrap();

    //static

    static mut VAR: i32 = 5;
    // 创建一个新线程
    let new_thread = thread::spawn(move || {
        unsafe {
            VAR = 10;
            println!("static value in new thread: {}", VAR);
        }
    });

    // 等待新线程先运行
    //    new_thread.join().unwrap();
    unsafe {
        println!("static value in main thread: {}", VAR);
    }

    //堆
    let var: Arc<i32> = Arc::new(5);
    let share_var = var.clone();
    let new_thread = thread::spawn(move || {
        println!("share value in new thread: {}, address: {:p}", share_var, &*share_var);
    });
    // 等待新建线程先执行
    new_thread.join().unwrap();
    println!("share value in main thread: {}, address: {:p}", var, &*var);

    //锁，通知
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        let (ref lock, ref cvar) = *pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
        println!("notify main thread");
    });

    let (ref lock, ref cvar) = *pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        println!("before wait");
        started = cvar.wait(started).unwrap();
        println!("after wait");
    }
    //原子 AtomicUsize

    let var: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(5));

    let share_var = var.clone();

    let new_thread = thread::spawn(move || {
        println!("share value in new thread : {}", share_var.load(Ordering::SeqCst));
        //修改值
        share_var.store(9, Ordering::SeqCst);
    });

    new_thread.join().unwrap();
    println!("share value in main thread: {}", var.load(Ordering::SeqCst));

    //Mutex
    let var: Arc<Mutex<u32>> = Arc::new(Mutex::new(5));
    let share_var = var.clone();

    // 创建一个新线程
    let new_thread = thread::spawn(move || {
        let mut val = share_var.lock().unwrap();
        println!("share value in new thread: {}", *val);
        // 修改值
        *val = 9;
    });

    // 等待新建线程先执行
    new_thread.join().unwrap();
    println!("share value in main thread: {}", *(var.lock().unwrap()));

    println!("===================concurrent test   end===================");
}