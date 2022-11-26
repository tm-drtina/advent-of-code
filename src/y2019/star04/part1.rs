fn is_valid(num: i32) -> bool {
    let num5 = num % 10;
    let num4 = num / 10 % 10;
    let num3 = num / 100 % 10;
    let num2 = num / 1000 % 10;
    let num1 = num / 10_000 % 10;
    let num0 = num / 100_000 % 10;
    let non_decreasing =
        num0 <= num1 && num1 <= num2 && num2 <= num3 && num3 <= num4 && num4 <= num5;
    let t1 = num0 == num1;
    let t2 = num1 == num2;
    let t3 = num2 == num3;
    let t4 = num3 == num4;
    let t5 = num4 == num5;

    non_decreasing && (t1 || t2 || t3 || t4 || t5)
}

pub fn run(input: &str) -> usize {
    let (min, max) = input.split_once('-').expect("valid input");
    let min = min.parse::<i32>().unwrap();
    let max = max.parse::<i32>().unwrap();
    (min..max).filter(|&i| is_valid(i)).count()
}
