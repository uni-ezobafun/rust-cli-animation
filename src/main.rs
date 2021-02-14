mod animation;

fn main() {
  const START_MESSAGE: &str = "now animation starts";
  println!("{}", START_MESSAGE);
  animation::start()
}
