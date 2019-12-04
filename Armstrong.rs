use std::io;

fn main()
{
  let mut n = String::new();
  println!("Enter a number: ");
  io::stdin().read_line(&mut n).unwrap();

  match n.trim_end().parse::<i32>() 
  {
    Ok(i) => check(i),
    Err(_) => println!("Invalid number."),
  }
  
}

fn check(mut n: i32)
{
  let mut rem;
  let mut sum = 0;
  let temp = n;

  while n > 0 
  {
  rem = n % 10 ;
  sum = sum + (rem * rem * rem);
  n = n / 10;
  }

  if temp == sum 
  {
    println!("{} is an Armstrong number",temp);
  }
  else
  {
  println!("{} is not an Armstrong number ",temp);
  }

}
