
pub fn iterator_test(){
    println!("===================iterator test begin===================");

    let v = vec![1, 2, 3, 4, 5];

    {
        let result = match IntoIterator::into_iter(v) {
            mut iter => loop {
                match iter.next() {
                    Some(x) => { println!("{}", x); },
                    None => break,
                }
            },
        };
        result
    }

    println!("===================iterator test   end===================");

}