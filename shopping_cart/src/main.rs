use std::io;

fn main() {
    println!("What would you like added to the cart?");

  let mut items: Vec<String> = Vec::new();

loop {
  let mut item = String::new();
  io::stdin().read_line(&mut item)
    .expect("failed to read item, please try again");

  if item.trim() == "exit" {
    break;
  }else {
    println!("Adding {} to your cart", item.trim() );
    items.push(item);
    println!("Please add a new item.");
  }
}
  println!("In your cart is");

  for item in &items {
    println!("{}", item.trim());
  }
}
