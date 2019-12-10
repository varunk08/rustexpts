fn main() {
    println!("Branches");

    let number = 3;

    if number < 5 {
	println!("Number is less");
    } else {
	println!("number is more");
    }

    let mut num = 10;

    while num != 0 {
	num = num - 1;
	println!("Number: {}", num);
    }

    let months = ["jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec"];

    num = 0;
    while num != 12 {
	println!("Month {} is {}", num, months[num]);
	num = num + 1;
    }

    for element in months.iter() {
	println!("Month: {}", element);
    }

    for index in (0..10).rev() {
	println!("{}", index);
    }
}
