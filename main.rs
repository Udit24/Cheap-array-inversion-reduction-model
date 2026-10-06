use std::io;
use rand::Rng;
const SIZE: usize=30;
fn pullforce(sample_array: &[i32])->(f64){
    //max min
    let mut largest:i32=sample_array.iter().max().copied().unwrap();
    let mut smallest:i32=sample_array.iter().min().copied().unwrap();
    let boundary: f32=(largest+smallest)/2;
    
    println!("The largest is {}, and the smallest is {},{},{}",largest,smallest,largest-smallest.abs(),((largest-smallest.abs()) as f64).exp());
    1.0 / (1.0 + (((-(largest-smallest.abs())) as f64).exp()))
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
    println!("returned {}",returnvalue);
    //calculation pull force on the fly to save itereation
    //
}

