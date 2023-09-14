fn main() {

    let x = 37.0;

    let result = if x > 20.0 && x < 50.0 {
        celcius_to_fahrenheit(x)
    } else {fahrenheit_to_celcius(x)};

    println!("The result is  -> {result}");

    let result = fibonacci(5);
    println!("The result of fibonacci is  -> {result}");

    let a = [1, 2, 3, 4, 5];
    let result = binary_search(a, 1);
    println!("Does element found in array -> {result}");
    
}

// Practise 1
fn fahrenheit_to_celcius(temperature: f64) -> f64 {
    (temperature - 32.0) * (5.0 / 9.0)
}

fn celcius_to_fahrenheit(temperature: f64) -> f64 {
    temperature * (9.0 / 5.0) + 32.0
}

// Practise 2
fn fibonacci(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;

    let mut x = n-2;
    let mut temp = 0;
    while x != 0 {
        temp = a+b;
        a = b;
        b = temp;
        x -= 1;
    }

    temp
}

// Practise 3
fn binary_search(arr: [i32; 5], x: i32) -> i32 {
    let mut start = 0;
    let mut end = arr.len()-1;
    
    let mut res = 0;
    let mut flag = false;
    while start <= end {
        let mid = (start+end) / 2;

        if arr[mid] == x {
            flag = true;
            res = mid;
        }

        if arr[mid] > x {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }

    let res: i32 = res.try_into().unwrap();
    if flag {
        res
    } else {
        -1
    }
}