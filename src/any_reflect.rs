use std::any::Any;
use std::fmt::Debug;

fn load_config<T: Any + Debug>(value: &T) -> Vec<String> {
    let mut cfgs: Vec<String> = vec![];
    let value = value as &Any;
    /*match value.downcast_ref::<String>() {
        Some(cfg) => cfgs.push(cfg.clone()),
        None => (),
    }
    match value.downcast_ref::<Vec<String>>() {
        Some(v) => cfgs.extend_from_slice(&v),
        None => (),
    }*/

    if let Some(cfg) = value.downcast_ref::<String>() {
        cfgs.push(cfg.clone())
    } else if let Some(v) = value.downcast_ref::<Vec<String>>() {
        cfgs.extend_from_slice(&v)
    }

    if cfgs.len() == 0 {
        panic!("No Config File");
    }
    cfgs
}

pub fn any_reflect_test() {
    let cfp = "/etc/wayslog.conf".to_string();
    assert_eq!(load_config(&cfp), vec!["/etc/wayslog.conf".to_string()]);
    let cfps = vec!["/etc/wayslog.conf".to_string(), "/etc/wayslog_sec.conf".to_string()];
    assert_eq!(load_config(&cfps), vec!["/etc/wayslog.conf".to_string(), "/etc/wayslog_sec.conf".to_string()]);
}