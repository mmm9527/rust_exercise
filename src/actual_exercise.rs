use serde::{Deserialize, Serialize};

// 定义TestStruct
#[derive(Debug, Serialize, Deserialize)]
pub struct TestStruct {
    data_int: u8,
    data_str: String,
    data_vector: Vec<u8>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ComplexStruct {
    data_int: u8,
    date_struct: Box<TestStruct>,
}

pub fn actual_exercise() {
    let object = TestStruct {
        data_int: 1,
        data_str: "homura".to_string(),
        data_vector: vec![2, 3, 4, 5],
    };

    // Serialize using `json::encode`
    // 将TestStruct转意为字符串
    // let encoded = json::encode(&object).unwrap();
    let encoded = serde_json::to_string(&object).unwrap();
    println!("{}", encoded);
    // 将json字符串中的数据转化成TestStruct对应的数据，相当于初始化
    // let decoded: TestStruct = json::decode(&encoded).unwrap();
    let decoded: TestStruct = serde_json::from_str(&encoded).unwrap();
    println!("{:?}", decoded);

    let c_object = ComplexStruct{
        data_int: 1,
        date_struct: Box::new(object),
    };



    let encoded = serde_json::to_string(&c_object).unwrap();
    println!("{}", encoded);

    let decoded: ComplexStruct = serde_json::from_str(&encoded).unwrap();
    println!("{:?}", decoded);

}