fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers: [i32; 10] = [ 2, 20, 14, 7, 9, 11, 15, 5, 8, 4];

    for num in numbers {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
     } else if num % 3 == 0 {
            println!("{}: Fizz", num);
     } else if num % 5 == 0 {
            println!("{}: Buzz", num);
     } else if is_even(num) {
            println!("{}: Even", num);
     } else {
            println!("{}: Odd", num);
     } 
}

let mut index = 0;
let mut sum = 0;

while index < numbers.len(){
    sum += numbers[index];
    index += 1;
}


println!("Sum of numbers: {}", sum); 

let mut i = 0;
let mut largest = numbers[0];

loop {
    if numbers [i] > largest {
        largest = numbers[i];
    }
    i += 1;

    if i >= numbers.len() {
        break;
    }
}
println!("Largest number: {}", largest);

}



