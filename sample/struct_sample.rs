#[derive(Debug)]
pub struct Data{

    pub num1: i32,
    pub num2: i32,
    pub str1: String,
    pub optional_num: Option<i32>

}

pub struct TwoNums(pub i32, pub i32);

pub struct Calculator;

impl Data
{
    pub fn sum(&self) -> i32{
        self.num1 + self.num2
    }
}

pub trait Transform
{
    fn rev(&self) -> String;

    fn output_rev(&self)
    {
        println!("{}", self.rev());
    }
}

impl Transform for Data{
    fn rev(&self) -> String
    {
        self.str1.chars().rev().collect()
    }
}