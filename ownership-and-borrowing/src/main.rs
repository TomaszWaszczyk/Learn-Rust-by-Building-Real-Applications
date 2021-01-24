use crate::ownership_and_borrowing::ownership_and_borrowing::take;

// mod ownership_and_borrowing;

fn main() {
    let x = 1; // x owns 1 
    // 1 is on a heap
    // let y = x;

    // scope
    {
        let a = 10;
    }
    // ==================================

    let s = String::from("String");
    // let y = s; it does not work

    // borrow s variable for y at the moment
    let y = &s; // borrowing

    println!("{}", s);

    let mut v = Vec::new();

    for i in 1..100{
        v.push(i);
    }

    ownership_and_borrowing::take(v);
}
