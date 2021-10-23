use std::mem;
 
 
 fn main()
{
    let mut numbers:[i32;4]=[2,3,4,5];
    numbers[2]=40;
    println!("{:?}",numbers);
    println!("single value:{}",numbers[0]);
    println!("array length:{}",numbers.len());
    println!("Array taking {} bytes",mem::size_of_val(&numbers));
    let slice: &[i32]=&numbers[0..1];
    println!("slice:{:?}",slice);
}