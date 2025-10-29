

fn take_ownership(vec: Vec<i32>) {
    println!("Vector inside function: {:?}", vec);
}

fn  give_ownership() -> Vec<i32> {
    vec![10,20,30]
}


fn take_and_return_ownership(mut vec: Vec<i32>) ->Vec<i32> {
    vec.push(10);
    vec
}

fn stack_function( mut var: i32) {
    var = 55;
    println!("In this function, Var is: {var}");

}


fn main() {
//    function that takes ownership //
    let vec_1: Vec<i32> = vec![1, 2, 3, 4, 5];

      take_ownership(vec_1.clone());

      println!("Vector in main after ownership moved: {:?}", vec_1); // This will throw compile error without the .clone() method.


    //   function that gives ownership //

    let vector_2: Vec<i32> = give_ownership();

    println!("vector 2 is: {:?} ", vector_2);

    // Function that takes and return ownership //

    let vector_3: Vec<i32> = take_and_return_ownership(vector_2);

    // println!("the vector 2 is: {:?} ", vector_2); // this fails because no ownership here the value have been move to vector_3
    println!("the vector 3 is: {:?} ", vector_3);



    // function that takes a stack data

    let num:i32 = 20;
    stack_function(num);
    println!("In the main, num is: {num}");
}
  
