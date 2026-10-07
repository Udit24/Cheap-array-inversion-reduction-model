use std::io;
use rand::Rng;
const SIZE: usize=30;
fn pullforce(sample_array: &[i32])->(f64){
    //max min
    let mut largest:i32=sample_array.iter().max().copied().unwrap();
    let mut smallest:i32=sample_array.iter().min().copied().unwrap();
    let boundary: f32=(largest+smallest) as f32/2.0;
    let mut temp_array=[sample_array.len()];
    //calculating mean
    let sum: i64  = sample_array.iter().sum() as i64;
    let mean: i64 = sum / sample_array.len() as i64;
    //calculate standard div
    let variance: f64 = sample_array.iter()
                            .map(|value|{
                                let diff=mean-(*value as i64);
                                diff*diff
                            }).sum();
    println!("The largest is {}, and the smallest is {},{},{}",largest,smallest,largest-smallest.abs(),((largest-smallest.abs()) as f64).exp());
    (variance/(sample_array.len()-1)) as f64
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

