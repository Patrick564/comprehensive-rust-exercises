mod day01;
mod day02;

fn main() {
    day01::morning::day01_morning();
    day01::afternoon::day02_morning();

    println!("---");

    day02::morning01::exercise();
    day02::afternoon01::luhn("1234 0000 0000 5678");
}
