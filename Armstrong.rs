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
  let mut sum = 0;
  let mut l=0;
  let mut rem;
  let temp = n;

   while n != 0 
    {
        n /= 10; 
        l = l + 1;
    }
  n = temp;
  while n > 0 
  {
  rem = n % 10 ;
  sum = sum + power(l , rem);
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

fn power(mut len: i32, rem: i32)->i32
{
  let mut p = 1;
  while len != 0 
  {
    p = p * rem ;
    len = len - 1;
  }
  return p;
}

