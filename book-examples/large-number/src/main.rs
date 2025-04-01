fn main() {
    let numbers = vec![43, 12, 53, 64, 23, 65, 13];
    println!("Large number:{}", largest_number(&numbers));
}

fn largest_number(numbers: &Vec<i32>) -> i32 {
    let mut large_number = &numbers[0];

    for number in numbers {
        if number > large_number {
            large_number = number;
        }
    }
    *large_number

}