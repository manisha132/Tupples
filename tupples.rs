fn main()
{
    let person:(&str,&str,i8)=("man","less",78);
    println!("{} is from {} and is {}",person.1,person.0,person.2);
}