//mod mystack;
use std::io::stdin;
use std::collections::VecDeque;

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
pub fn complex_arithmetic() {
	println!("All input values must be integers, and please put a space (\" \") between each value and operator.\nPlease enter a calculation:");
	let equation = get_input();
	let eq_vector : Vec<&str> = equation.split(" ").collect();
	let rpn = shuntingyard(eq_vector);
	reverse_polish_notation(0, rpn.clone());
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
		//we include the parentheses only for shuntingyard
	}else if op.trim()=="(" {
		1
	}else if op.trim()==")" {
		0
	}else {
		-1
	}
}
//this is a shunting-yard algorithm, it is used for converting infix to rpn, which is much easier than the inverse
fn shuntingyard(eq : Vec<&str>) -> String {
	let length = eq.len();
	let mut deque : VecDeque<&str> = VecDeque::new();
	let mut operators : Vec<&str> = vec![];
	for i  in 0..length {
		//println!("i = {}", i);
		let token = eq[i].clone();
		let prec = get_precedence(token);
		if prec < 0  {
			deque.push_back(token);
		}else if prec > 1 {
			if operators.len() < 1 {
				//println!("Pushed {} to stack", token);
				operators.push(token);
			}else {
				loop {
					let currtoken = operators.pop().unwrap();
					if get_precedence(currtoken) >= prec && get_precedence(currtoken) > 1 {
						//println!("Moved {} to queue", currtoken);
						deque.push_back(currtoken);
					}else {
						//println!("Replaced {} on stack", currtoken);
						operators.push(currtoken);
						break;
					}
				}
				//println!("Pushed {} to stack", token);
				operators.push(token);
			}
		}else if prec == 1 {
			//println!("We got left parentheses");
			operators.push(token);
		}else if prec == 0 {
			loop {
				let currop = operators.pop().unwrap();
				if currop == "(" {
					break;
				}else if operators.len() == 0{
					println!("Uhhhh, phrasing...");
					break;
				}else {
					deque.push_back(currop);
				}
			}
		}else {
			println!("Uhhhh, phrasing...");
		}
	}
	//println!("We formed the queue!");
	//now for clearing the operators
	loop {
		if operators.len() < 1 {
			break;
		}
		deque.push_back(operators.pop().unwrap());
	}
	//println!("We made it past the second loop");
	let mut rpn = String::from(deque[0]);
	for i in 1..deque.len(){
		rpn = [&*rpn.clone(), deque[i]].join(" ")
	}
	//println!("we made it past here mate.");
	rpn
}
//THIS FUNCTION DOES NOT WORK, but it does compile! So it can be useful for learning the algorithm:
fn rpntoinfix(mut rpn: Vec<&str>) -> String {
	let mut alg_stack: Vec<String> = vec![];
	let mut precedence: Vec<i32> = vec![];
	let mut i = 0;
	let len = rpn.len();
	loop {
		if i >= len {
			break;
		}
		let val;
		if rpn.len() == 0 {
			val = String::from(rpn.pop().unwrap());
		}else {
			val = alg_stack.pop().unwrap();
		}
		if get_precedence(&*val) > -1 && precedence.len() >= 2 {
			let prec = get_precedence(&*val);
			let num2;
			let prec2 = precedence.pop().unwrap();
			if prec2 > -1 {
				precedence.push(prec2);
				alg_stack.push(String::from(&*val));
				precedence.push(prec);
				continue;
			}
			let num1;
			let prec1 = precedence.pop().unwrap();
			if prec2 > -1 {
				precedence.push(prec1);
				precedence.push(prec2);
				alg_stack.push(String::from(&*val));
				precedence.push(prec);
				continue;
			}
			let insideval2 = alg_stack.pop().unwrap();
			if prec2 > -1 && prec2 < prec {
				num2 = [String::from("("),insideval2 , String::from(")")].join("");
			} else {
				num2 = String::from(insideval2);
			}
			let insideval1 = alg_stack.pop().unwrap();
			if prec1 > -1 && prec1 < prec {
				num1 = [String::from("("), insideval1, String::from(")")].join("");
			} else{
				num1 = String::from(insideval1);
			}
			alg_stack.push([num1, val, num2].join(" "));
			precedence.push(prec);		
		} else {
			precedence.push(get_precedence(&*val));
			alg_stack.push(val);
		}
		i+=1;
	}	
	alg_stack.pop().unwrap()
}
/*fn reverse_stack<'a>(normal: String) -> Vec<&'a str> {
	let mut norm: Vec<&str> = normal.split(" ").collect();
	let mut new: Vec<&str> = vec![];
	for i in 0..norm.len() {
		new.push(norm.pop().unwrap());
	}
	new
}*/
pub fn reverse_polish_notation(input: i32, repn: String){
	let mut rpn;
	if input == 1 {
		println!("Please enter your polish notation string:\nPlease insert a space (\" \") between each item in your input");
		rpn = get_input();
		rpn.pop();
	}else {
		//println!("{}", repn);
		rpn = repn;
	}
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
	let mut value;	
	for i in 0..normal.len() {
		value = normal.pop().unwrap();
		if get_precedence(value) == -1 {
			let num = value.parse::<i32>();
			stack.push(num.unwrap());
		}else {
			let num1 = stack.pop().unwrap();
			let num2 = stack.pop().unwrap();
			stack.push(do_op(num2, num1, value));
		}
	}
	println!("Answer: {}", stack.pop().unwrap());
}
pub fn help(){
	println!("MathTools Help: \nFor help; type \"help\"\nFor simple arithmetic(multiplication, addition, subtraction, exponents, etc), type \"simple\"\nFor Reverse polish notation, type \"rpn\"\nFor complex calculations invloving order of operations: type \"complex\"\nTo exit, type \"exit\"");
}
