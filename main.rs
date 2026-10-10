use std::io;
use rand::Rng;
const SIZE: usize=10000;
const HSIZE: usize=163;

fn main(){
    //init a demo array to be changed later
    let mut rng = rand::thread_rng();
    let vec: Vec<i32> = (0..SIZE).map(|_| rng.random()).collect();
    /*Having problems using a normal array in fear of running out of memory space
    a heap is much safer to prevent running out of memory than a stack*/
    /*Looping to choose exactly 163 random elements from the vector to use as
    pivots. Storing their indexes in an ind array[163], and the elements in ele array[163]*/
    let mut ind:[u32;HSIZE]=[0 as u32;HSIZE];let mut ele:[i32;HSIZE]=[0 as i32;HSIZE];
    for i in 0..163{
        ind[i] = rand::random_range(0..SIZE) as u32;
        ele[i]=vec[ind[i] as usize] as i32;  
        //println!("At index {:?}, the value {:?}",ind[i],ele[i]);
    }
    let mut temp_vec: Vec<i32> = Vec::with_capacity(SIZE);
    for item in &vec{
        let mut count:usize=0;
        for i in 0..163usize{
            if(*item<vec[i]){
                count+=1;
            }
        }
        let rank_estimate:usize=(1+((vec.len()-1)/163)*count) as usize;
        temp_vec.insert(rank_estimate,*item);
    }
    //println!("ARRAY:{:?}",vec);
}

