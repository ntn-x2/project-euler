pub fn main() {
    let sum = (1u32..1000)
        .filter(|n| n % 3 == 0 || n % 5 == 0)
        .sum::<u32>();
    println!("Sum: {sum}");
}
