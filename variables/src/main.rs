fn main()
{
    //noral variable declaration is immutable by default
    let x = 10;
    println!("Immutable variable x has value: {}", x);

    //We can make it mutable using mut
    let mut y = 10;
    println!("Value of y before mutation: {}", y);

    y = 20;
    println!("Value of y after mutation: {}", y);
}
