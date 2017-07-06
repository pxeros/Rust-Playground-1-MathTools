//mod mystack;
use std::io::stdin;

pub fn get_input() -> String {
	let mut buffer = String::new();
	stdin().read_line(&mut buffer).expect("Failed");
	buffer
}
pub fn do_thing(){
	println!("Holy Shit, the call worked.");
}
pub fn simple_arithmetic() {
	println!("Please enter a line following this pattern:\nNumber operator Number");
	let equation = get_input();
	let eq_vector : Vec<&str> = equation.split(" ").collect();
	if eq_vector.len()!=3 {
		println!("Try again");
	}
	let num1 = eq_vector[0].trim().parse::<i32>().unwrap();
	let num2 = eq_vector[2].trim().parse::<i32>().unwrap();
	let num3 = do_op(num1, num2, eq_vector[1]);
	println!("{one} {two} {three} = {four}", one = num1, two = eq_vector[1], three = num2, four = num3);
}
fn do_op(num1: i32, mut num2: i32, oper: &str) -> i32 {
	let mut result = 0;
	let op = oper.trim();
	if op=="+" {
		result = num1 + num2;
	}else if op=="*" {
		result = num1 * num2;
	}else if op=="-" {
		result = num1 - num2;
	}else if op=="/" {
		result = num1/num2;
	}else if op=="%" {
		result = num1%num2;
	}else if op=="^" {
		result = 1;
		while num2>0 {
			result = result*num1;
			num2-=1;
		}
	}else{
		println!("Not a valid operation");
	}
	result
}
fn get_precedence(op: &str) -> i32 {
	if op.trim()=="^" {
		4
	}else if op.trim()=="*" || op.trim()=="/" || op.trim()=="%" {
		3
	}else if op.trim()=="+" || op.trim()=="-" {
		2
	}else {
		return -1;
	}
}
//This function will be coming soon: It will print the postfix expression into infix
/*fn rpntoinfix(rpn: Vec<&str>) -> String {
	let mut alg_stack: Vec<&str> = vec![];
	let mut alg_string = String::from("");
	let mut precedence: Vec<i32> = vec![];
	for i in 0..rpn.len() {
		let num = rpn.peek().parse::<i32>().unwrap();
		match num {
			Ok(num) {alg_stack.push(rpn.pop()); 
					 continue;
					}
		}
		precedence.push(get_precedence(rpn.peek()));
		let val1 = alg_stack.pop();
		let val2 = alg_stack.pop();
		let short = String::from("");
		short.push(String::from(val1.trim()));
		short.push(String::from(rpn.pop().trim()));
		short.push(String::from(val2.trim()));
		alg_stack.push(&short);
	}
}*/
/*fn reverse_stack<'a>(normal: String) -> Vec<&'a str> {
	let mut norm: Vec<&str> = normal.split(" ").collect();
	let mut new: Vec<&str> = vec![];
	for i in 0..norm.len() {
		new.push(norm.pop().unwrap());
	}
	new
}*/
pub fn reverse_polish_notation(){
	println!("Please enter your polish notation string:\nPlease insert a space (\" \") between each item in your input");
	let mut rpn = get_input();
	rpn.pop();
	//reversing the stack
	let mut norm: Vec<&str> = rpn.split(" ").collect();
	let mut new: Vec<&str> = vec![];
	for i in 0..norm.len() {
		new.push(norm.pop().unwrap());
	}
	//store it
	let mut normal: Vec<&str> = new;
	let mut stack: Vec<i32> = vec![];
	//This was a cool idea I had, I will print the rpn string as infix
	//let mut norm_string = rpntoinfix(normal);
	let mut value = "";	
	for i in 0..normal.len() {
		value = normal.pop().unwrap();
		if get_precedence(value) == -1 {
			let num = value.parse::<i32>();
			stack.push(num.unwrap());
		}else {
			let num1 = stack.pop().unwrap();
			let num2 = stack.pop().unwrap();
			println!("{} {} {}", num2, value, num1);
			stack.push(do_op(num2, num1, value));
		}
	}
	println!("Answer: {two}", two = stack.pop().unwrap());
}
pub fn help(){
	println!("MathTools Help: \nFor help; type \"help\"\nFor simple arithmetic(multiplication, addition, subtraction, exponents, etc), type \"simple\"\nFor Reverse polish notation, type \"sausage\"\nTo exit, type \"exit\"");
}
