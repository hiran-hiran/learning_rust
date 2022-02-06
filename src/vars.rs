pub mod sub_a;
pub mod sub_b;

const MAX_POINT: u32 = 100_000;

pub fn run() {
  println!("Here is vars module");

  // sub_a::func_a();
  // sub_b::func_b();

  // let mut x = 5;
  // println!("x is {}", x);

  // x = 6;
  // println!("x is {}", x);

  println!("{}", usize::BITS);
  println!("memory const addres is {:p}", &MAX_POINT);

  let i2: i64 = 1;
  let i3: i64 = 2;
  println!("stack i2 addres is {:p}", &i2);
  println!("stack i3 addres is {:p}", &i3);

  // shadowing
  let y = 5;
  println!("stack y is {:p}", &y);
  let y = y + 2;
  println!("stack y is {:p}", &y);
  let y = y * 2;
  println!("stack y is {:p}", &y);
  println!("stack y is {}", y);

  // tuple
  let t1 = (500, 6.4, "slice text");
  println!("t1 is {} {} {}", t1.0, t1.1, t1.2);

  let mut t2 = ((0, 1), (2, 3));
  let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
  *x1_ptr = 5;
  *y1_ptr = -5;
  println!("{:?}", t2);

  // array
  let a1 = [1, 2, 3, 4, 5];
  let a2 = [0; 10];
  println!("{:?} {:?} {} {}", a1, a2, a1[2], a1[3]);
}
