fn main() {
    //NUMBER TYPE

    //integers--------------------------------------------//
    let _x: i8 = -50;
    let _y: u8 = 255;

    //floating-point types--------------------------------//
    let _a = 2.7; //f64 by default
    let _b: f32 = 2.0;

    //math operations ------------------------------------//

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;

    // remainder
    let _remainder = 43 % 5;

    //boolean---------------------------------------------//

    let _t = true;
    let _f: bool = false; // with explicit type annotation

    //CHARACTER TYPE

    let _c = 'z';
    let _z = 'Z';

    //tuple type
    let _tup: (i32, f64, u8) = (500, 6.4, 1); //the annotations are optionals

    let (_x2, _y2, _z2) = _tup;

    let five_hundred = _tup.0;
    println!("{}", five_hundred);

    //array type
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let _first = _months[0];
    let _second = _months[1];
}
