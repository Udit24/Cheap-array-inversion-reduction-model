use std::io;
use rand::Rng;
const SIZE: usize=30;


fn pullforce(sample_array: &[i32]){
    //max min
    let mut largest:i32=sample_array.iter().max().copied().unwrap();
    let mut smallest:i32=sample_array.iter().min().copied().unwrap();
    let boundary: f32=(largest+smallest) as f32/2.0;
    let mut temp_array=[sample_array.len()];
    //calculating mean
    let sum: i32  = sample_array.iter().sum();
    let mean: i32 = sum / <usize as TryInto<i32>>::try_into(sample_array.len()).expect("Index out of bounds");
    
}
fn main(){
    //init a demo array to be changed later
    let mut sample_array: [i32; SIZE] = rand::random();
    println!("Array initialised as: {:?}", sample_array);
    //calculate StrengthA, StrengthB
    //loop
    //pullforce function call
    //this is changed
    //also this
    let mut returnvalue=pullforce(&sample_array);
    println!("returned {:?}",returnvalue);
    //calculation pull force on the fly to save itereation
    //
}

