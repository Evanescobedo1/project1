///
/// @Authors Carson Burley Evan Escobedo
/// @Date Mar 2 2023
/// 
/// 
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, Write, Error};
use std::process::exit;

///
/// This runs the program and handles cmd errors
/// 
fn main() {
    //Check command line input
    let args: Vec<String> = args().collect();
    if args.len() < 3{
        println!("Usage: cargo run ./src/input.txt ./output.txt");
        exit(1);
    }
    //Get file names
    let in_file: &str = &args[1];
    let out_file: &str = &args[2];

    //Create expression list
    let mut expr_list: Vec<Expression> = build_expression_list(&in_file).expect("Could not find the input file.");

    //Solve list of expressions and sort
    solve_list(&mut expr_list);
    sort_list(&mut expr_list);
    
    //Write to a file
    write_to_file(&out_file, expr_list).expect("Could not write to the file.");//TODO catch expect

}
///Builds an expression list from an input file
///
///  # Arguments 
///  * 'in_file' - A string slice containing the file path
/// 
///  # Returns
///  *Result (vec<expression>, string) either a vector of expressions, or an error message
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

///
/// Runs through a vec of expression and calls the individual solve functions 
/// of the struct
/// 
/// # Arguments
/// * exp_list a list of expression structs
/// 
fn solve_list(exp_list: &mut Vec<Expression>){
    for i in exp_list{
        i.solve();
    }
}

///
/// Writes the to_string() of a vector of expresssions to a specified file
/// 
/// # Arguments 
/// * out_file a string containing the name of a file path to write to 
/// * expr_list a vector of expressions 
/// 
/// # Returns 
/// * Result()
/// 
fn write_to_file(out_file: &str, expr_list: Vec<Expression>) -> Result<(), Error>{
    let mut output = File::create(out_file).unwrap();
    for expr in &expr_list{
        let output_line = expr.to_string() + "\n";
        output.write_all(output_line.as_bytes())?;
    }
    Ok(())
}

///
/// Struct to model an expression to convert from postfix to infix
/// 
/// #Fields
/// *postfix a string containing the read postfix notation expression
/// *expr a stack used to hold the running total of the expression
/// *infix a vector of strings containing infix notation expression
struct Expression {
    //The original expression.
    postfix: String,
    //Results of expression.
    expr: Vec<f64>,
    //Expression after being converted to infix notation.
    infix: Vec<String>,
}

///
/// Implimentation of expression
/// 
/// #Methods
/// *expression(string) taking a string creates the instance of an expression
/// *solve converts the postfix field to infix notation and solves the expression
/// *get_result() returns the result of the expression once evaluated
/// *to_string() returns the expression in a string representation
impl Expression {
    
    ///
    /// Constructor for expression instances 
    /// #Arguments 
    /// *line a line representing the postfix expression
    /// 
    fn expression(line: String) -> Self{
        Expression{postfix: line, expr: Vec::new(), infix: Vec::new()}
    }
    
    ///
    /// Method to convert the postfix expression to infix
    /// 
    fn solve(&mut self){
        //Seperates each digit or operator in the expression from whitespace
        let temp = self.postfix.split_whitespace();
        //Checks how many operators were found in a row (More than 2 is an invalid expression)
        //let mut op_check: i32 = 0;
        for i in temp{
            //When a number is found, push it onto the stack
            if is_string_number(i) && self.infix.len() < 4{
                self.expr.push(i.parse::<f64>().expect("wasnt a num"));
                self.infix.push(i.to_string());
                
            //When an operator is found, grab the last 2 numbers and push them
            //to the expr stack after performing the operation.
            } else if operator_check(i) && self.expr.len() > 1{
                let num2 = self.expr.pop().unwrap();
                let num1 = self.expr.pop().unwrap();
                self.expr.push(do_math(num1, num2, i));

                let expr2 = self.infix.pop().unwrap();
                let expr1 = self.infix.pop().unwrap();
                self.infix.push(build_infix(&expr1.to_string(), &expr2.to_string(), &i.to_string()));

               
            //Catches invalid expressions
            } else{
                println!("Malformed Expression");
                exit(1);
            }

        }
        //Catches incomplete expressions (too few operators)
        if self.infix.len() != 1{
            println!("Malformed Expression");
            exit(1);
        }
        
    }

    ///
    /// Returns the result of the expression within this struct
    ///used for sorting
    /// 
    fn get_result(&self) -> &f64{
        return self.expr.get(0).unwrap();
    }

    ///
    /// Returns a string representation of the expression 
    /// 
    /// #Return
    /// *String the expression represented as {infix} = {expr result}
    /// 
    fn to_string(&self) -> String{
        let mut out: String = self.infix.get(0).unwrap().to_string() + " = ";
        out = out + &self.expr.get(0).unwrap().to_string();
        return out;
    }

}
///
/// Helper method to handle math operations when performing the algorithm to convert
/// the postfix to infix
/// 
/// #Arguments 
/// *num1 a number 
/// *num2 another number
/// *operator the operation meant to be performed on the pair
/// 
/// #Returns 
/// *f64 the result of the sub expression
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

///
/// Helper method to make checking strings easy
/// 
/// #Arguments
/// *str a string to be checked
/// 
/// #Return
/// *bool the result from trying to parse the number
/// 
fn is_string_number(str: &str) -> bool {
    let _i = match str.parse::<f64>(){
        Ok(_i) => return true,
        Err(_e) => return false,
    };
}


/// 
/// Helper method to create the infix expression to be pushed onto the 
/// infix stack
/// 
/// 
/// #Arguments
/// *num1 a string representation of a number from the infix stack
/// *num2 a string representation of a number from the infix stack
/// *operator an operator from the postfix string
/// 
/// #Returns
/// *String representation of the two number or expressions combined with the operator
/// 
fn build_infix(num1: &String, num2: &String, operator: &String) -> String{
    let mut out = String::new();

    //Checks to see if parentheses might be necessary on the original expressions
    if operator == "*" || operator == "/"{
        //Find the center of the first expression, if it is a + or - parentheses are needed
        let mut split_string: Vec<&str> = num1.split(" ").collect();
        let mut mid_op = split_string.get(split_string.len() / 2).unwrap();

        if ! is_string_number(num1) &&  mid_op == &"+" || mid_op == &"-"{
            out = "( ".to_owned() + num1;
            out =  out + " )";
        }else{
            out = out + num1;
        }
        out = out + " " + operator + " ";
        
        //Repeat for the second expression
        split_string = num2.split(" ").collect();
        mid_op = split_string.get(split_string.len() / 2).unwrap();
        if ! is_string_number(num2) &&  mid_op == &"+" || mid_op == &"-"{
            out = out + "( ";
            out = out + num2;
            out = out + " )";
        }else{
            out = out + num2;
        }
    }else{
        out = num1.to_owned() + " " + operator + " " + num2;
    }
    
    return out;
}


///
/// Helper method to handle checking for unknown characters in the 
/// postfix input
/// 
/// will exit with message "Unknwon Operator: {}" displaying the detected
/// symbol
/// 
/// #Arguments
/// *str the operator to be questioned
/// 
/// #Returns 
/// *bool the operator is known or not
fn operator_check(str: &str) -> bool{
    //Check to see if the supplied string is more than one char
    if str.len() != 1{
        return false
    }
    //Check the string to see if its a number
    if is_string_number(str){
        return false
    }
    //Match statement for the string
    match str
    {
        "+"|"-"|"*"|"/" => return true,
        //Panic Case and message
        _ => {
            println!("Unknown Operator: {}", str);
            exit(1)
        },

    }
}


///
/// Selection sort will sort a supplied vector in ascending order
/// 
/// #Arguments 
/// *vector a vector to be sorted
/// 
/// 
fn sort_list(vector: &mut Vec<Expression>){
    for num in 0..vector.len(){
        let lowest: usize = find_lowest(vector, num);

        if lowest > num{
            vector.swap(num, lowest);
        } 
    }
}


///
/// Helper method to find the uszie of the the smallest element of the vec
/// 
/// #Arguments
/// *vector the vector being sorted
/// *start the current element being sorted
/// 
/// #Return 
/// *usize the index of the smallest vector element
fn find_lowest(vector: &mut Vec<Expression>, start: usize) -> usize {
    let mut lowest: usize = start;
    for num in start + 1..vector.len(){
        if vector[num].get_result() < vector[lowest].get_result(){
            lowest = num;
        }
    }
    return lowest;
}
