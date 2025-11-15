let val1: i32 = 10;
let val2 = 10;
let val3: i64 = -16;

let val4: u32 = 20;
let val5: u8 = 255;

let val6 = 1234i32;
let val7 = 1234_u32;

let val8: i64 = 1_234_567;

let val9: f32 = 3.14;
let val10: f64 = 2.155445;

let val11: bool = true;
let val12: bool = false;

let val13: char = 'a';
let val14: char = 'A';
let val15: char = '♥️';

let tup: (i32, f64, char) = (500, 6.4, 'j');
let five_hundred = tup.0;
let six_point_four = tup.1;
let letter_j = tup.2;
let (val1, val2, val3) = tup;

let nums: [i32, 5];
nums[0] = 10;
let nums: [i32; 5] =[1,2,3,4,5];
let first = nums[0];
let second = nums[1];

let mut nums = vec![1,2,3,4,5];
let firts = nums[0];
nums.push(6);

struc Person{
    name: String,
    age: u8
}
let alice = Person{
    name: "Alice".toString(), 
    age:23
}
let alice_age = alice.age;
let Person {name, age} = alice;

enum Choice{
    A,
    B,
    C
}

let b = Choice::B;

let mut hello = String::from("Hello, ");
helo.push_str("World");

let x = 5;

let mut y = 10;
const MAX_POINTS: U32 = 100_000;

fn add(value1: i32,  value2: i32) -> i32{
    value1 + value 2
}

fn greet(name: &str){
    println!("Hello, {}", name);
}

let a = 10;
let b = 20;

a == b;
a != b;

a < b;
a > b;
a <= b;
a >= b;

let x = true;
let y = false;

x && y

x || y

!x

let number =6;
if number <5
{
    println!("small");
}else
{
    println!("large");
}

let result = if number > 5 {"big"} else {"small"};

let number =3;
match number{
    1 => println!("One");
    2 => println!("Two");
    3 => println!("Three");
    4..10 => println!("between 4 an 10");
    _ => println!("SOmething else");

}

let mut counter = 0;

loop
{
    counter += 1;
    println!("Value {}", counter);
    if counter == 3
    {
        break;
    }
}

let result = loop
{
    counter += 1;
    if counter == 10
    {
        break counter * 2;
    }
}

let mut number = 3;
while number != 0
{
    println!("{}", number);
    number -= 1;
}

for i in 1..4
{
    println!("{}", i);
}

for i in 1..=4
{
    println!("{}", i);
}

for i in (0..10).step_by(2)
{
    i += 2;
}

let arr = [10, 20, 30, 40, 50];
for element in arr
{
    println!("Element: {}", element);
}

for (index, value) in arr.iter().enumerate()
{
    println!("Index {}, {}", index, value);
}

'outer: for i in 1..5
{
    'iiner for j in 1..5
    {
        if i == 2 && j == 2
        {
            break 'outer;
        }
    }
}