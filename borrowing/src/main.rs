

//  the function //
fn borrow_vec(vec: &Vec<i32>) {
    println!("Vector inside function: {:?}", vec);
}


fn borrow_vec_mut(vec: &mut Vec<i32>) {
    vec.push(40);
    println!("Vector inside mutable borrow function: {:?}", vec);
}

fn takes_and_give_ownership( mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(40);
    vec
}


fn mixed_borrow(subject: &String, scores: &mut Vec<i32>) {
    println!("{} Before the Update: {:?}", subject, scores);
    scores.push(99);
    println!("{} After Update: {:?} ", subject, scores);
}

fn main() {
    let vector_1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let ref1: &Vec<i32> = &vector_1;
    let ref2: &Vec<i32> = &vector_1;
    println!("Reference 1: {:?}, Refrence 2: {:?}", ref1, ref2);

    // borowing in functions //
    // 1. function that immutably borrows values //
    let vec_1: Vec<i32> = vec![10, 20, 30];
    // let reference: &Vec<i32> = &vec_1; // immutable reference
    borrow_vec(&vec_1);
    println!("Vector in main after borrowing: {:?}", vec_1);


    // 2. function that mutably borrows values //
    let vec_2: &mut Vec<i32> = &mut vec![100, 200, 300];

    borrow_vec_mut(vec_2);
    println!("Vector in main after mutable borrowing: {:?}", vec_2);

    // another scenario

    let vector_2: Vec<i32> = vec![500, 600, 800];

    let vector_2: Vec<i32> = takes_and_give_ownership(vector_2);  //shadowing solves this problem
    println!("the Vector_2 is: {:?}", vector_2);



    // functions that uses mixed types to borrow //

    let mut scores: Vec<i32> = vec![80, 90, 70];
    let subject:String = String::from("Maths");
    mixed_borrow(&subject, &mut scores);

    println!("The scores are: {:?} ", scores);

}

