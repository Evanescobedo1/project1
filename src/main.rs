use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, Write, Error};
use std::process::exit;
fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 3{
        println!("Usage: cargo run ./src/input.txt ./output.txt");
        exit(1);
    }
    let in_file: &str = &args[1];
    let out_file: &str = &args[2];
    let mut expr_list: Vec<Expression> = build_expression_list(&in_file).expect("Could not find the input file.");
    solve_list(&mut expr_list);
    sort_list(&mut expr_list);
    write_to_file(&out_file, expr_list).expect("Could not write to the file.");//TODO catch expect

}
///Builds an expression list from an input file
///
///  # Arguments 
///  * 'in_file' - A string slice containing the file path
/// 
///  # Returns
///  *Result
/// 
fn build_expression_list(in_file: &str) -> Result<Vec<Expression>, &'static str>{
    //create the vector that holds the Expressions
    let mut expr_list: Vec<Expression> = Vec::new();
    //open the file for reading
    let file = File::open(in_file).expect("Could not read file");
    //make a buffered reader for the file
    let reader = BufReader::new(&file);

    //read in each line and push to expr_list
    for line in reader.lines(){
        //Right here ignore blank lines
        let line_read = &line.expect("Line was unreadable");
        if !&line_read.trim().is_empty(){
            expr_list.push(Expression::expression(line_read.to_string()));
        }
    }
    if expr_list.len() == 0{
        println!("Empty or Blank Space File");
        exit(1)
    }
    //return expr_list wrapped in a result
    Ok(expr_list)
}

fn solve_list(exp_list: &mut Vec<Expression>){
    for i in exp_list{
        i.solve();
    }
}

fn write_to_file(out_file: &str, expr_list: Vec<Expression>) -> Result<(), Error>{
    let mut output = File::create(out_file).unwrap();
    for expr in &expr_list{
        let output_line = expr.to_string() + "\n";
        output.write_all(output_line.as_bytes())?;
    }
    Ok(())
}

struct Expression {
    //The original expression.
    postfix: String,
    //Results of expression.
    expr: Vec<f64>,
    //Expression after being converted to infix notation.
    infix: Vec<String>,
}

impl Expression {
    fn expression(line: String) -> Self{
        Expression{postfix: line, expr: Vec::new(), infix: Vec::new()}

    }
    //Method to convert the postfix expression to infix
    fn solve(&mut self){
        //Seperates each digit or operator in the expression from whitespace
        let temp = self.postfix.split_whitespace();
        //Checks how many operators were found in a row (More than 2 is an invalid expression)
        let mut op_check: i32 = 0;
        for i in temp{
            //When a number is found, push it onto the stack
            if is_string_number(i) && self.expr.len() <= 3{
                self.expr.push(i.parse::<f64>().expect("wasnt a num"));
                op_check = 0;
                
            //When an operator is found, grab the last 2 numbers and push them
            //to the expr stack after performing the operation.
            } else if operator_check(i) && op_check < 2 && self.expr.len() > 0{
                let num2 = self.expr.pop().unwrap();
                let num1 = self.expr.pop().unwrap();
                self.expr.push(do_math(num1, num2, i));
                op_check += 1;

                //When the last character checked was not an operator. Just create a new
                //expression and push it to the infix stack.
                if op_check == 1{
                    self.infix.push(build_infix(&num1.to_string(), &num2.to_string(), &i.to_string(), false));

                //When the last character checked was an operator. pop the last 2 expressions from the infix
                //stack and combine them, then push the new expression to the infix stack. If the operator is
                // * or /, add parenthises around the previous expressions.
                } else{
                    let expr2 = self.infix.pop().unwrap();
                    let expr1 = self.infix.pop().unwrap();
                    self.infix.push(build_infix(&expr1.to_string(), &expr2.to_string(), &i.to_string(), i == "*" || i == "/"));
                }
            //Catches invalid expressions
            } else{
                println!("Malformed Expression");
                exit(1);
            }

        }
        //Catches incomplete expressions (too few operators)
        if self.infix.len() != 1{
            println!("Malformed Expression");
            //exit(1);
        }
        
    }
    ///Returns the result of the expression within this struct
    ///used for sorting
    fn get_result(&self) -> &f64{
        return self.expr.get(0).unwrap();
    }

    //Returns a string representation of the expression 
    fn to_string(&self) -> String{
        let mut out: String = self.infix.get(0).unwrap().to_string() + " = ";
        out = out + &self.expr.get(0).unwrap().to_string();
        return out;
    }

}


fn do_math(num1: f64, num2: f64, operator: &str)-> f64{
    match operator{
        "+"=> return num1 + num2,
        "-"=> return num1 - num2,
        "*"=> return num1 * num2,
        "/"=> return num1 / num2,
        _  => {
            println!("Unknown Operator Encountered: {}", operator);
            exit(1);
        },
    }
}

//Helper method to make checking strings easy
fn is_string_number(str: &str) -> bool {
    let _i = match str.parse::<f64>(){
        Ok(_i) => return true,
        Err(_e) => return false,
    };
}


fn build_infix(num1: &String, num2: &String, operator: &String, add_parens: bool) -> String{
    let mut out = num1.to_owned();
    if add_parens{
        out = "( ".to_owned() + &out;
        out =  out + " ) ";
        out = out + operator;
        out = out + " ( ";
        out = out + num2;
        out = out + " )";
    }else{
        out = out + " ";
        out = out + operator;
        out = out + " ";
        out = out + num2;
    }
    return out;
}

fn operator_check(str: &str) -> bool{
    if str.len() != 1{
        return false
    }
    if is_string_number(str){
        return false
    }
    match str{
        "+"|"-"|"*"|"/" => return true,
        _ => {
            println!("Unknown Operator: {}", str);
            exit(1)
        },

    }
}


fn sort_list(vector: &mut Vec<Expression>){
    for num in 0..vector.len(){
        let lowest: usize = find_lowest(vector, num);

        if lowest > num{
            vector.swap(num, lowest);
        } 
    }
}

fn find_lowest(vector: &mut Vec<Expression>, start: usize) -> usize {
    let mut lowest: usize = start;
    for num in start + 1..vector.len(){
        if vector[num].get_result() < vector[lowest].get_result(){
            lowest = num;
        }
    }
    return lowest;
}
