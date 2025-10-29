

fn main() {
//    Dereferencing of stack allocated types

    let mut some_data:i32 = 45;

    let ref_1: &mut i32 = &mut some_data; 
    let deref_copy = *ref_1;
     *ref_1 = 30;
    println!("The Dereferences copy is: \n {}  \n and some_data value is: \n {} ", deref_copy, some_data);

    //  Dereferencing of heap allocated types //

    let mut heap_data: Vec<i32> = vec![10, 20, 30];

    let ref_2:&mut Vec<i32> = &mut heap_data;
    // let deref_copy: Vec<i32> = *ref_2;

    ref_2.push(60);
    (*ref_2).push(60);

    println!("The heap data after dereferencing is: {:?}", heap_data);




}
