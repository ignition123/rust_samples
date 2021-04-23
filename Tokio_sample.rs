// importing crate
use std::mem;
use std::env;
use std::fs::File;
use std::io::Write;
use std::sync::mpsc::channel;
use std::sync::mpsc::sync_channel;
use std::thread;
use std::sync::{Arc, Mutex, RwLock};
use std::collections::{HashMap, BTreeMap, HashSet, BTreeSet, BinaryHeap};
use std::collections::VecDeque;
use std::collections::LinkedList;
use std::cmp::Reverse;
use futures::executor::block_on;
use tokio::sync::{mpsc, oneshot, broadcast, watch};
use tokio_stream::StreamExt;

// creating a struct
struct Point{
    x: f64,
    y: f64,
}

// creating a struct
struct Rectangle{
    top_left: Point,
    bottom_right: Point,
}

// implementing Point with a function area
impl Point{
    fn area(&self) -> f64
    {
        self.x * self.y
    }
}

// implementing Drop Trait
impl Drop for Rectangle {
    fn drop(&mut self)
    {
        println!("Dropped");
    }
}

// implementing Drop Trait
impl Drop for Point {
    fn drop(&mut self)
    {
        println!("Dropped");
    }
}

// creating a origin function with Point struct as return Type
fn origin() -> Point{
    Point{x: 2.0, y:1.0}
}

// creating a empty struct
struct A;

// creating a struct with type A struct
struct Single(A);

// creating a struct with generic data type
struct SingleGen<T>(T);

// creating a struct with 3 fields
struct MyStruct {
    test_field: Option<String>,
    name: String,
    age: i32,
}

// implementing struct with a function named new which is return MyStruct as return type
impl MyStruct{
    pub fn new(new_age: i32, new_name: String) -> MyStruct {
        MyStruct {
            test_field: None,
            age: new_age,
            name: new_name,
        }
    }
}

// creating a enum
enum CarType {
    Hatch,
    Sedan,
    SUV
}

struct Movie {
    title: String,
    director: String,
    release_year: u32,
    genre: String
}

trait Details {
    fn description(&self) -> String;
    fn years_since_release(&self) -> u32;
}

impl Details for Movie{

    fn description(&self) -> String{
        return format!("{}, released in {}, is a {} movie directed by {}.", self.title, self.release_year, self.genre, self.director);
    }

    fn years_since_release(&self) -> u32 {
        return 2020 - self.release_year;
    }
}

fn get_an_optional() -> Option<&'static str>{
    return Some("Sudeep Dasgupta");

    None
}

fn sum() -> Result<u64, String>{
    return Err(String::from("Error in function"));

    Ok(255)
}

fn is_even(no:i32)->Result<bool,String> {
    if no%2==0 {
        return Ok(true);
    } else {
        return Err("NOT_AN_EVEN".to_string());
    }
}

#[tokio::main]
async fn main()
{
    let str= String::from("javaTpoint tutorial");
    let _java_point = &str[0..10];

    println!("{:?}", _java_point);

    // box smart pointers
    let five = Box::new(5);
    println!("{:?}", five);

    // stack allocated variables
    let point: Point = origin();

    let rectangle: Rectangle = Rectangle{
        top_left: origin(),
        bottom_right: Point{x:2.0, y:5.0}
    };

    // heap allocated variables
    let boxed_rectangle : Box<Rectangle> = Box::new(Rectangle{
        top_left: origin(),
        bottom_right: Point{x:7.0, y:8.0}
    });

    let box_point: Box<Point> = Box::new(origin());

    println!("{:?}", mem::size_of_val(&point));
    println!("{:?}", mem::size_of_val(&rectangle));
    println!("{:?}", mem::size_of_val(&boxed_rectangle));
    println!("{:?}", mem::size_of_val(&box_point));

    drop(box_point);

    let _s = Single(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('c');

    println!("{:?}", _i32.0);
    println!("{:?}", _char.0);

    let _my_struct = MyStruct{
        test_field: None,
        name: String::from("Sudeep Dasgupta"),
        age: 29,
    };

    println!("{}", _my_struct.name);

    println!("{}", MyStruct::new(29, String::from("Sudeep Dasgupta")).name);

    let car = CarType::SUV;

    match car {
        CarType::Hatch => {
            println!("Small sized car");
        },
        CarType::Sedan => {
            println!("medium sized car");
        },
        CarType::SUV =>{
            println!("Large sized Sports Utility car");
        }
    }

    let movie1 = Movie{
        title: "Titanic".to_string(),
        director: "James Cameron".to_string(),
        release_year: 1997,
        genre: "historical".to_string()
    };

    println!("{}", movie1.description());

    println!("The movie was released {} years ago.", movie1.years_since_release());

    println!("{:?}", match get_an_optional(){
        Some(T) => println!("Abhishek Roy"),
        None => println!("no data found"),
    });

    let home_path = env::home_dir();

    match home_path{
        Some(T) => println!("{:?}", T),
        None => println!("No path found"),
    }

    println!("{:?}", match sum(){
        Ok(T) => println!("{:?}", T),
        Err(e) => println!("{:?}", e),
    });

    println!("{:?}", get_an_optional().is_none());

    let f = File::open("main.jpg");
    //this file does not exist
    match f {
        Ok(f)=> {
            println!("file found {:?}",f);
        },
        Err(e)=> {
            println!("file not found \n{:?}",e);   //handled error
        }
    }
    println!("end of main");

    let result = is_even(13);

    match result {
        Ok(d)=>{
            println!("no is even {}",d);
        },
        Err(msg)=>{
            println!("Error msg is {}",msg+" hello world");
        }
    }

    println!("end of main");

    let mut f = File::create("ferris.txt");  // Returns Result<File>

    // To unwrap we match on the enum variants
    let mut file = match f {
        Ok(f) => f,
        Err(e) => panic!("File could not be created")
    };

    match file.write_all(String::from("Hi Ferris").as_ref()) {
        Ok(_) => {},
        Err(e) => panic!("Could not write to file")
    }

    let mut file = File::create("ferris.txt").unwrap();
    file.write_all("Hi Ferris".as_ref()).unwrap();

    let mut file = File::create("ferris1.txt")
        .expect("Could not create file `ferris.txt`");

    file.write_all("Hi Ferris".as_ref()).expect("Could not write to file");

    // creating a channel in rust
    let (sender, receiver) = channel();

    // Spawn off an expensive computation
    thread::spawn(move || {
        sender.send(1 + 2).unwrap();
    });

    // Do some useful work for awhile

    // Let's see what that answer was
    println!("{:?}", receiver.recv().unwrap());

    // creating a sync channel
    let (sender, receiver) = sync_channel(1);

    // this returns immediately
    sender.send(1).unwrap();

    thread::spawn(move|| {
        // this will block until the previous message has been received
        sender.send(2).unwrap();
    });

    println!("{:?}", receiver.recv().unwrap());
    println!("{:?}", receiver.recv().unwrap());

    let (tx, rx) = channel();

    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(i).unwrap();
        });
    }

    for _ in 0..10 {
        let j = rx.recv().unwrap();
        println!("{:?}", 0 <= j && j < 10);
    }

    let data = Arc::new(Mutex::new(0));

    let (tx, rx) = channel();

    const N: usize = 10;

    for _ in 0..N {
        let (data, tx) = (Arc::clone(&data), tx.clone());
        thread::spawn(move || {
            // The shared state can only be accessed once the lock is held.
            // Our non-atomic increment is safe because we're the only thread
            // which can access the shared state when the lock is held.
            //
            // We unwrap() the return value to assert that we are not expecting
            // threads to ever fail while holding the lock.
            let mut data = data.lock().unwrap();
            *data += 1;
            if *data == N {
                tx.send(()).unwrap();
            }
            // the lock is unlocked here when `data` goes out of scope.
        }).join();
    }

    println!("{:?}", rx.recv().unwrap());

    let data = Arc::new(Mutex::new(HashMap::new()));

    for val in 0..N {

        let data = Arc::clone(&data);

        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data.insert(String::from("name")+ &*val.to_string(), "sudeep dasgupta");

        }).join();
    }

    let mut data = data.lock().unwrap();

    println!("{:?}", data.get("name0"));

    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("{:?}", vec.len());

    let mut vec1 = vec![1, 2, 3];

    for x in &vec1 {
        println!("{}", x);
    }

    let mut buf = VecDeque::new();
    buf.push_back(3);
    buf.push_back(4);
    buf.push_back(5);


    if let Some(elem) = buf.get_mut(1) {
        *elem = 7;
    }

    println!("{:?}", buf.get(1));

    let mut list1 = LinkedList::new();
    list1.push_back('a');

    let mut list2 = LinkedList::new();
    list2.push_back('b');
    list2.push_back('c');

    list1.append(&mut list2);

    let mut iter = list1.iter();

    println!("{:?}", iter.next());

    let mut book_reviews = HashMap::new();

// Review some books.
    book_reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book.".to_string(),
    );
    book_reviews.insert(
        "Grimms' Fairy Tales".to_string(),
        "Masterpiece.".to_string(),
    );
    book_reviews.insert(
        "Pride and Prejudice".to_string(),
        "Very enjoyable.".to_string(),
    );
    book_reviews.insert(
        "The Adventures of Sherlock Holmes".to_string(),
        "Eye lyked it alot.".to_string(),
    );

    if !book_reviews.contains_key("Les Misérables") {
        println!("We've got {} reviews, but Les Misérables ain't one.",
                 book_reviews.len());
    }

    // oops, this review has a lot of spelling mistakes, let's delete it.
    book_reviews.remove("The Adventures of Sherlock Holmes");

    let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
    for &book in &to_find {
        match book_reviews.get(book) {
            Some(review) => println!("{}: {}", book, review),
            None => println!("{} is unreviewed.", book)
        }
    }

    for (book, review) in &book_reviews {
        println!("{}: \"{}\"", book, review);
    }

    let mut movie_reviews = BTreeMap::new();

    // review some movies.
    movie_reviews.insert("Office Space",       "Deals with real issues in the workplace.");
    movie_reviews.insert("Pulp Fiction",       "Masterpiece.");
    movie_reviews.insert("The Godfather",      "Very enjoyable.");
    movie_reviews.insert("The Blues Brothers", "Eye lyked it a lot.");

    if !movie_reviews.contains_key("Les Misérables") {
        println!("We've got {} reviews, but Les Misérables ain't one.",
                 movie_reviews.len());
    }

    let mut books = HashSet::new();

    // Add some books.
    books.insert("A Dance With Dragons".to_string());
    books.insert("To Kill a Mockingbird".to_string());
    books.insert("The Odyssey".to_string());
    books.insert("The Great Gatsby".to_string());

    // Check for a specific one.
    if !books.contains("The Winds of Winter") {
        println!("We have {} books, but The Winds of Winter ain't one.",
                 books.len());
    }

    let mut books = BTreeSet::new();

    // Add some books.
    books.insert("A Dance With Dragons");
    books.insert("To Kill a Mockingbird");
    books.insert("The Odyssey");
    books.insert("The Great Gatsby");

    // Check for a specific one.
    if !books.contains("The Winds of Winter") {
        println!("We have {} books, but The Winds of Winter ain't one.",
                 books.len());
    }

    let mut heap = BinaryHeap::new();

    heap.push(1);
    heap.push(5);
    heap.push(2);

    println!("{:?}", heap.peek());
    println!("{:?}", heap.pop());

    let mut heap = BinaryHeap::new();

    // Wrap values in `Reverse`
    heap.push(Reverse(1));
    heap.push(Reverse(5));
    heap.push(Reverse(2));

    println!("{:?}", heap.pop());

    let future = hello_world();
    block_on(future);

    block_on(learn_and_sing());

    let op = say_world();

    // This println! comes first
    println!("hello");

    // Calling `.await` on `op` starts executing `say_world`.
    op.await;

    thread::spawn(move || {
        let mut rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            println!("hello sudeep");
        });
    }).join();

    let handle = tokio::spawn(async {
        // Do some async work
        "return value"
    });

    let out = handle.await.unwrap();
    println!("GOT {}", out);

    let v = vec![1, 2, 3];

    let v = v.clone();

    tokio::spawn(async move{
        println!("Here's a vec: {:?}", v);
    });

    // tokio mpc channel
    let (tx, mut rx) = mpsc::channel(100);

    tokio::spawn(async move {
        for i in 0..10 {
            if let Err(_) = tx.send(i).await {
                println!("receiver dropped");
                return;
            }
        }
    });

    while let Some(i) = rx.recv().await {
        println!("got = {}", i);
    }

    // tokio oneshot channel
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        if let Err(_) = tx.send(3) {
            println!("the receiver dropped");
        }
    });

    match rx.await {
        Ok(v) => println!("got oneshot = {:?}", v),
        Err(_) => println!("the sender dropped"),
    }

    // tokio broadcast
    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();

    tokio::spawn(async move {
        println!("{:?}", rx1.recv().await.unwrap());
        println!("{:?}", rx1.recv().await.unwrap());
    });

    tokio::spawn(async move {
        println!("{:?}", rx2.recv().await.unwrap());
        println!("{:?}", rx2.recv().await.unwrap());
    });

    tx.send(10).unwrap();
    tx.send(20).unwrap();

    // tokio watch
    let (tx, mut rx) = watch::channel("hello");

    tokio::spawn(async move {
        while rx.changed().await.is_ok() {
            println!("received = {:?}", *rx.borrow());
        }
    });

    tx.send("world");

    // async channels
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        let _ = tx1.send("one");
    });

    tokio::spawn(async {
        let _ = tx2.send("two");
    });

    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    }

    let mut stream = tokio_stream::iter(&[1, 2, 3]);

    while let Some(v) = stream.next().await {
        println!("GOT = {:?}", v);
    }

    // let messages = stream
    //     .into_stream()
    //     .filter(|msg| match msg {
    //         Ok(msg) if msg.content.len() == 1 => true,
    //         _ => false,
    //     })
    //     .map(|msg| msg.unwrap().content)
    //     .take(3);
}

async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    let song = learn_song().await;

    println!("{:?}", song);
}

async fn learn_song() -> String{

    String::from("sudeep dasgupta!!!")
}

async fn hello_world() {
    println!("hello, world!");
}

async fn say_world() {
    println!("world pritha");
}
