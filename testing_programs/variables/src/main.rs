fn main() {
 let mut x = 3;
{
    let multiplier = x * x;
    println!("{multiplier}");
}
 x = 1;
{
    let additive = x + x + x;
    println!("{additive}")

}

}
