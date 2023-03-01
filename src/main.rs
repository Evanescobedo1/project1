use std::env::args;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write, Error};
use std::process::exit;
fn main() {
    let mut expr_list: Vec<Expression> = build_expression_list("./src/input.txt").unwrap();
    solve_list(expr_list);
}

fn build_expression_list(in_file: &str) -> Result<Vec<Expression>, &'static str>{
    //create the vector that holds the Expressions
    let mut expr_list: Vec<Expression> = Vec::new();
    //open the file for reading
    let file = File::open(in_file).expect("Could not read file");
    //make a buffered reader for the file
    let reader = BufReader::new(&file);
    //read in each line and push to expr_list
    for line in reader.lines(){
        expr_list.push(Expression::expression(line.unwrap()));
    }
    //return expr_list wrapped in a result
    Ok(expr_list)
}

fn solve_list(exp_list: Vec<Expression>){
    for mut i in exp_list{
        i.solve();
    }
}

//fn sort_list(exp_list: Vec<Expression>){
    
//}

//fn write_to_file(out_file: &str, exp_list: Vec<Expression>) -> Result<Vec<Expression>, &'static str>{
    
//}

struct Expression {
    postfix: String,
    expr: Vec<f64>,
    infix: Vec<String>,
}

impl Expression {
    fn expression(line: String) -> Self{
        Expression{postfix: line, expr: Vec::new(), infix: Vec::new()}

    }
    fn solve(&mut self){
        let mut stack: Vec<String>= Vec::new();
        let temp = self.postfix.split_whitespace();
        for i in temp{
            println!("{}", is_string_number(i));
            println!("Value is: {:?}",i);

        }
        println!("______________________");
    }
}

//Helper method to make checking strings easy
fn is_string_number(str: &str) -> bool {
    for c in str.chars(){
        if !c.is_numeric(){
            return false;
        }
    }
    return true;
}