use std::io::Write;

fn main() {
    let mut vec1 = Vec::new();
    write!(&mut vec1, "test");
    println!("{:?}", vec1);
}
// [116, 101, 105, 116]
