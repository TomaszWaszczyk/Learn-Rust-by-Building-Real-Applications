fn main() {
    println!("Hello, world!");

    
    let v: Vec<i32> = vec![1,2,34,5];
    println!("{:?}", v);
    v.into_iter().map(|x| x + 1).rev().collect();
    println!("{:?}", v)
    /*
    #![allow(unused)]
    fn main() {
    let v: Vec<i32> = vec![1, 2, 3].into_iter().map(|x| x + 1).rev().collect();

    assert_eq!(v, [4, 3, 2]);
    } */
}
    
