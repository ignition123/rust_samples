#![allow(dead_code)]

use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::io;
use std::collections::HashMap;
use rand::{thread_rng, Rng};

use std::process::Command;

extern crate regex;
use regex::Regex;

extern crate reqwest;

extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;
use serde_json::Value as JsonValue;

pub mod sample;
use self::sample::struct_sample::Transform;

struct People{

    name: String,
    age: u8

}

trait HasVoiceBox{

    // speak
    // check if can speak

    fn speak(&self);
    fn can_speak(&self) -> bool;
}

impl HasVoiceBox for People{

    fn speak(&self)
    {
        println!("My name is {}", self.name);
    }

    fn can_speak(&self) -> bool{

        if self.age > 0
        {
            return true;
        }
        else
        {
            return false;
        }

    }

}

struct Color{

    red: u8,
    green: u8,
    blue: u8,

}

struct Rectangle
{
    width: u32,
    height: u32,
}

impl Rectangle
{
    fn print_rectangle(&self)
    {
        println!("{}", self.width * self.height)
    }

    fn is_square(&self) -> bool
    {
        self.width == self.height
    }
}

struct Person
{
    name: String,
    age: u8
}

impl ToString for Person
{
    fn to_string(&self) -> String
    {
        return format!("My name is {} and I am {}", self.name, self.age);
    }
}

mod decoder
{
    pub fn print_message()
    {
        println!("How it is going");
    }

    pub mod water
    {
        pub fn print_message()
        {
            println!("I am water");
        }
    }
}

fn get_occupation(name: &str) -> Option<&str>
{
    match name{
        "Domenic" => Some("Software Developer"),
        "Michael" => Some("Dentist"),
        _=> None
    }
}

enum Day
{
    Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday
}

impl Day
{
    fn is_weekday(&self) -> bool
    {
        match self
        {
            &Day::Saturday | &Day::Sunday => return false,
            _ => return true
        }
    }
}

// test cases

#[cfg(test)]
mod dcode_tests
{
    #[test]
    #[should_panic]
    fn test_basic()
    {
        assert!(1 == 1); // OK
        panic!("Oh no!");
    }

    #[test]
    #[ignore]
    fn test_equals()
    {
        assert_eq!(2, 1 + 1);
        assert_ne!(2, 1 + 2);
    }
}

fn main(){

    let x = 45;

    let mut y = 45;

    const Z:i32 = 50;

    println!("###################################### Variables example");

    println!("{}", x);

    println!("{}", y);

    y = 46;

    println!("{}", y);

    println!("{}", Z);

    println!("Hello, decode!");

    let _company_string = "TutorialsPoint";  // string type
   	let _rating_float = 4.5;                 // float type
   	let _is_growing_boolean = true;          // boolean type
    let _icon_char = '♥';   
       
    println!("{}", _company_string);
    println!("{}", _rating_float);
    println!("{}", _is_growing_boolean);
    println!("{}", _icon_char);

    let a:u128 = 45;

    println!("{}", a);

    let b:bool = false;

    println!("{}", b);

    println!("###################################### Variables example");


    println!("####################################### conditional statements");

    if x > 45
    {
        println!("x is greater than equals to 45");
    }
    else
    {
        println!("x is less than 45");
    }

    match x {

        45 => println!("equals to 45"),
        2 | 3 | 5 | 7 | 11 => println!("is a even number"),
        _ => println!("Ain't special"),

    }

    println!("####################################### conditional statements");

    println!("####################################### loops");

    let mut n = 0;

    loop{

        n += 1;

        println!("The value of in is {}", n);

        if n > 10{
            break;
        }
        else
        {
            continue;
        }

    }

    let mut f = 1;

    while f < 100
    {
        println!("value of f is {}", f);

        f += 1;
    }

    for _g in 0..100
    {
        println!("value of f is {}", _g);
    }

    println!("####################################### loops");

    println!("####################################### enums");

    enum Direction{

        Up,
        Down,
        Left,
        Right

    }

    let player_direction:Direction = Direction::Up;

    match player_direction{

        Direction::Up => println!("We are heading up"),
        Direction::Down => println!("We are heading down"),
        Direction::Left => println!("We are heading left"),
        Direction::Right => println!("We are heading right")

    }

    let name = String::from("Domenic");

    println!("Character at index 8: {}", match name.chars().nth(8){

        Some(c) => c.to_string(),
        None => "No character at index 8!".to_string()

    });

    println!("Occupation is {}", match get_occupation("Domenic") {

        Some(o) => o,
        None => "No occupation found!"

    });

    let d = Day::Tuesday;

    println!("Is d a weekday: {}", d.is_weekday());

    println!("####################################### enums");

    println!("####################################### Tuples");

    let tup1 = (20, 25, 30 , 35);

    println!("{}", tup1.2);

    let tup2 = (20, true, "Rust" , 3.5, (1,4,6));

    println!("{}", tup2.2);

    println!("{}", (tup2.4).0);

    println!("####################################### Tuples");

    println!("####################################### Functions");

    print_numbers_to(30);

    println!("{}", is_even(30));

    println!("####################################### Functions");

    println!("####################################### Code block");

    let mut h = 10;

    {   
        let u = 10;

        h = 20;

        println!("{}", h);
    }

    println!("{}", h);

    println!("####################################### Code block");

    println!("####################################### Shadowing block");

    let mut x = 10;

    {
        let x = 15;

        println!("{}", x);
    }

    println!("{}", x);

    println!("####################################### references");

    let mut x = 10;

    let xr = &mut x;

    *xr += 1;

    println!("{}", x);

    {
        let dom = &mut x;

        *dom += 1;
    }

    println!("{}", x);

    println!("####################################### references");

    println!("####################################### Struct");

    let mut bg = Color{
        red: 255,
        green: 0,
        blue: 15,
    };

    println!("{},{},{}", bg.red, bg.green, bg.blue);

    bg.red = 101;

    println!("{}", bg.red);

    struct ColorTupple(u8, u8, u8);

    let mut red = ColorTupple(255, 0, 0);

    red.2 = 50;

    println!("{}", red.2);

    print_color(&mut bg);

    println!("{}", bg.red);

    println!("####################################### Struct");

    println!("####################################### Arrays");

    let numbers = [1, 2, 3, 4, 5];

    for n in numbers.iter()
    {
        println!("{}", n)
    }

    for n in 0..numbers.len()
    {
        println!("{}", numbers[n])
    }

    let number1: [i32; 5] = [1, 2, 3, 4, 5];

    for i in number1.iter()
    {
        println!("{}", i);
    }

    let number3: [i32; 5] = [10;5];

    for i in number3.iter()
    {
        println!("{}", i);
    }

    println!("####################################### Arrays");

    println!("####################################### impl");

    let my_rect = Rectangle{
        width: 10,
        height: 10,
    };

    my_rect.print_rectangle();

    println!("{}", my_rect.is_square());

    println!("####################################### impl");

    println!("####################################### Strings");

    let mut my_string = String::from("How's it going? My name is Dom.");

    println!("String {}", my_string);

    println!("Length :{}", my_string.len());

    println!("Is empty :{}", my_string.is_empty());

    for token in my_string.split_whitespace()
    {
        println!("{}", token)
    }

    println!("Does the string contains :{}", my_string.contains("Sudeep"));

    my_string.push_str("Welcome to your tutorial on Strings");

    println!("New string added: {}", my_string);

    println!("####################################### Strings");

    println!("####################################### Traits");

    let dom = Person{

        name: String::from("Domenic"),
        age: 21,

    };

    println!("{}", dom.to_string());


    let people = People{

        name: String::from("Sudeep Dasgupta"),
        age: 28

    };

    println!("Can {} speark ? {}", people.name, people.can_speak());

    println!("####################################### Traits");

    println!("####################################### Vectors");

    // let my_vector: Vec<i32> = Vec::new();

    let mut my_vector = vec![1, 2, 3, 4];

    println!("{}", my_vector[2]);

    my_vector.push(49);

    my_vector.remove(1); /// remove 2

    for number in my_vector.iter()
    {
        println!("{}", number);
    }

    println!("####################################### Vectors");

    println!("####################################### File Reader");

    let mut file = File::open("info.txt").expect("Can't open file!");

    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Oops ! can not read the file...");

    println!("{}", contents);


    println!("####################################### File Reader");

    println!("####################################### Command line arguments");

    let args: Vec<String> = env::args().collect();

    for argument in args.iter()
    {
        println!("{}", argument);
    }

    println!("####################################### Command line arguments");

    println!("####################################### Write File");

    let mut file = File::create("output.txt").expect("Could not create file!");

    file.write_all(b"Welcome to dcode!").expect("Cannot write to the file");

    println!("####################################### Write File");

    println!("####################################### match pattern matching");

    let number = 2;

    match number{

        1 => println!("It is one..."),
        2 => println!("There is two of them"),
        _ => println!("It does not match"),

    };

    println!("####################################### match pattern matching");

    println!("####################################### input string");

    let mut input = String::new();

    println!("Hey mate! Say something: ");

    match io::stdin().read_line(&mut input)
    {
        Ok(_) => {

            println!("Success! you said: {}", input.to_uppercase());

        },
        Err(e) => println!("Oops, something went wrong: {}", e)
    }


    println!("####################################### input string");


    println!("####################################### Hashmaps");


    let mut marks = HashMap::new();

    // add values

    marks.insert("Rust Programming", 96);

    marks.insert("Web Development", 72);

    marks.insert("UX Design", 80);

    marks.insert("Professional Computing Studies", 87);

    println!("Total keys: {}", marks.len());

    match marks.get("Web Development")
    {
        Some(mark) => println!("You got {} for web dev ", mark),
        None => println!("You did not study web development"),
    }

    marks.remove("UX Design");

    for (subject, mark) in &marks{

        println!("For {} you got {}",subject, mark);
    } 

    println!("Did you study c++ {}", marks.contains_key("c++ Programming"));


    println!("####################################### Hashmaps");

    println!("####################################### Rand");

    let random_number = rand::thread_rng().gen_range(1, 11); // 1-10

    println!("Random number {}", random_number);

    // Flip a coin

    let random_bool = rand::thread_rng().gen_weighted_bool(2);

    println!("{}", random_bool);

    println!("####################################### Rand");

    println!("####################################### String Methods");

    {
        let my_string = String::from("Rust is fantastic!");

        println!("After replace: {}", my_string.replace("fantastic", "great"));        
    }

    /* Lines */

    {

        let my_string = String::from("The weather is \nnice\noutside mate!");

        for line in my_string.lines()
        {
            println!("print line [ {} ]", line);
        }

    }

    /* Split */

    {

        let my_string = String::from("Leave+a+like+if+you+enjoyed!");

        let tokens: Vec<&str> = my_string.split("+").collect();

        for token in tokens.iter()
        {
            println!("token {}", token);
        }

    }

    /* Trim */

    {

        let my_string = String::from(" My name is Domenic \r\n");

        println!("my_string {}", my_string);

        println!("trim my_string {}", my_string.trim());

    }

    /* Chars */

    {

        let my_string = String::from("decode in Youtube");

        /* Get character at index */

        match my_string.chars().nth(4){

            Some(c) => println!("Character at index 4: {}", c),
            None => println!("No character at index 4")

        };

    }

    println!("####################################### String Methods");

    println!("####################################### Call Modules");

    sample::decode::print_message();

    decoder::print_message();

    decoder::water::print_message();

    println!("####################################### Call Modules");

    println!("####################################### Regex");

    let re = Regex::new(r"\w{5}").unwrap();

    let text = "dcode";

    println!("Found match {}", re.is_match(text));

    match re.captures(text)
    {
        Some(caps)=>println!("Found match: {}", caps.get(0).unwrap().as_str()),
        None => println!("Could not found match...")
    };

    println!("####################################### Regex");

    println!("####################################### Http Get Request");

    match reqwest::get("http://206.189.132.17:8080/") {

        Ok(mut response) =>{

            // Check if 200 ok

            if response.status() == reqwest::StatusCode::Ok
            {   
                match response.text()
                {
                    Ok(text) => println!("Response Text: {}", text),
                    Err(e) => println!("Could not read renponse text {}", e)
                }
            }
            else
            {
                println!("Response was not 200 ok");
            }

        },
        Err(e) => println!("Could not make the request... {} ",e)

    };

    let response_text = reqwest::get("http://206.189.132.17:8080/")
        .expect("Could not make request")
        .text().expect("Could not read the response text");
    
    println!("Response Text: {}", response_text);

    println!("####################################### Http Get Request");

    println!("####################################### Command Line Interface");

    // python decode.py

    let mut cmd = Command::new("python");

    cmd.arg("decode.py");

    // execute the command

    match cmd.output()
    {
        Ok(o) =>{
            unsafe
            {
                println!("Output: {}", String::from_utf8_unchecked(o.stdout));
            };
        },
        Err(e) => println!("There was an error {}", e)
    }

    println!("####################################### Command Line Interface");

    println!("####################################### JSON parsing");


    let json_str = r#"

        {
            "name":"Domenic",
            "age":65,
            "is_male":true,
            "skills":[
                {
                    "name":"PHP"
                },
                {
                    "name":"Rust"
                }
            ]
        }

    "#;

    let res = serde_json::from_str(json_str);

    if res.is_ok()
    {
        let p: JsonValue = res.unwrap();
        println!("The name is {}", p["skills"][0]["name"]);
    }
    else
    {
        println!("Could not parse JSON");
    }

    #[derive(Serialize, Deserialize)]
    struct JsonPerson
    {
        name: String,
        age: u8,
        is_male: bool
    }

    let res = serde_json::from_str(json_str);

    if res.is_ok()
    {
        let p: JsonPerson = res.unwrap();
        println!("The name is {}", p.name);
    }
    else
    {
        println!("Could not parse JSON");
    }

    println!("####################################### JSON parsing");

    println!("####################################### Struct");

    let a = sample::struct_sample::Data{
        num1: 4,
        num2: 3,
        str1: "whatever".to_string(),
        optional_num: None
    };

    let d = sample::struct_sample::TwoNums(4, 3);

    println!("{} {}", d.0, d.1);

    println!("{:?}", a);

    println!("{:?}", a.sum());

    println!("{}", a.rev());

    a.output_rev();

    println!("####################################### Struct");

    println!("####################################### Struct Sample");

    let o = sample::struct_example::Object
    {
        width: 35,
        height: 55,
    };

    let obj = sample::struct_example::Object::new(47, 56);

    println!("{}*{} width area: {}", o.width, o.height, o.area());


    println!("####################################### Struct Sample");

}

fn print_color(c: &mut Color)
{
    println!("Color - R:{}, G:{}, B:{}", c.red, c.green, c.blue);

    c.red = 10;
}

fn print_numbers_to(num: u32)
{
    println!("{}", num)
}

fn is_even(num: u32) -> bool{

    num % 2 == 0

}