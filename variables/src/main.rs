fn main() {
    println!("Hello, world!");

    let x = 3.02342354;

    println!("The floating point number: {}", x);

    let sum =  5 + 10;
    let difference = 95.5 - 4.3;
    let remainder = 45 % 5;

    println!("Sum: {}, Diff: {}, Rem: {}", sum, difference, remainder);

    let t = true;
    let f: bool = false;

    println!("True: {}, False: {}", t, f);

    let c = 'z';
    let heart_eyed_cat = '😻';

    println!("Chars: {}, {}", c, heart_eyed_cat);

    // Compund types
    let tup: (i32, f64, u8) = (500, 6.4,  1);
    let (x,y,z) = tup;
    println!("Y = {}", y);

    let a = [1, 2, 4, 5, 6];
    println!("a[3] = {}", a[3]);
}
