fn main() {
    println!("Generics in Rust");

    let v = vec![1, 4, 21, 45, 10, 8];
    println!("The largest value in vector v is -> {}", largest(&v));

    let integer_point = Point{x: 1, y: 2};
    let float_point = Point{x: 3.0, y: 5.0};
    println!("The integerpoint and floatpoint are -> {:?} & {:?}", integer_point, float_point);
    println!("{}, {}", integer_point.x, integer_point.y);
}

// generics in function
// std::cmp::PartialOrd --> trait which restricts the generic type T
// to only be those types whose values be ordered.
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// generics in struct
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
