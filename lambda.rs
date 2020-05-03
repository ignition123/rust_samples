fn main()
{
    let v = vec![1,2,3,4,5];

    let y:Vec<i32> = v.iter().map(| i | i * i).collect();

    println!("{:?}", y);

    let z = v.iter().fold(0, | sum, i | sum + i);

    println!("{}", z);

    let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    let mean = v.iter().fold(0.0, | sum, i | sum + i) / v.len() as f32;

    println!("{}", mean);

    let std_dev = (v.iter()
        .map(|i| {(i-mean).powf(2.0)})
        .fold(0.0, | sum, i | sum + i) / v.len() as f32)
        .sqrt();

    println!("{}", std_dev);
}