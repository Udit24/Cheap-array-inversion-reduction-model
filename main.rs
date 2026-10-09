use std::io;
use rand::Rng;
const SIZE: usize=30;

fn find_median(sample_array: &[i32], mut i:usize,mut j:usize)->(i32){
    let boundary: usize=(i+j)/2 as usize;
    sample_array[boundary]
}
fn pullforce(sample_array: &[i32]){
    //max min
    let mut largest:i32=sample_array.iter().max().copied().unwrap();
    let mut smallest:i32=sample_array.iter().min().copied().unwrap();
    let boundary: f32=(largest+smallest) as f32/2.0;
    let mut temp_array=[sample_array.len()];
    //calculating mean
    let sum: i32  = sample_array.iter().sum();
    let mean: i32 = sum / <usize as TryInto<i32>>::try_into(sample_array.len()).expect("Index out of bounds");
    //The above methods do their jobs in logn time and I wish to change them in future
    //I want to use quick sorts median of three rule to find a pivot
    //I want to use the pivot to create small and less camp
    // since the push and pull needs to happen in at most longn (targetting loglogn) time, I split the data at median and subsequent medians into logn parts
    let median: i32=find_median(&sample_array,0,sample_array.len()-1);
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

