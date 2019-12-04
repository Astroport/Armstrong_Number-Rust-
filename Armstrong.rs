fn main(){
  let mut n = 153;
  let mut rem;
  let mut sum = 0;
  let temp;

 temp = n ;

while n > 0 {
  rem = n % 10 ;
  sum = sum + (rem * rem * rem);
  n = n / 10;
}

if temp == sum {
  println!("{} is an Armstrong number",temp);
}else{
  println!("{} is not an Armstrong number ",temp);
}

}
