fn is_valid(num: &i32) -> bool {
    let num5 = num % 10;
    let num4 = num / 10 % 10;
    let num3 = num / 100 % 10;
    let num2 = num / 1000 % 10;
    let num1 = num / 10000 % 10;
    let num0 = num / 100000 % 10;
    let non_decreasing =
        num0 <= num1 && num1 <= num2 && num2 <= num3 && num3 <= num4 && num4 <= num5;
    let t1 = num0 == num1;
    let t2 = num1 == num2;
    let t3 = num2 == num3;
    let t4 = num3 == num4;
    let t5 = num4 == num5;
    return non_decreasing && (t1 || t2 || t3 || t4 || t5);
}

pub fn run(min: i32, max: i32) -> usize {
    (min..max).filter(|i| is_valid(i)).count()
}
