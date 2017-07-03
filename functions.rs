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
	let mut num3 = 0;
	if eq_vector[1]=="+" {
		num3 = num1 + num2;
	}else if eq_vector[1]=="*" {
		num3 = num1 * num2;
	}else if eq_vector[1]=="-" {
		num3 = num1 - num2;
	}else if eq_vector[1]=="/" {
		num3 = num1/num2;
	}else if eq_vector[1]=="%" {
		num3 = num1%num2;
	}else{
		println!("Not a valid operation");
	}
	println!("{one} {two} {three} = {four}", one = num1, two = eq_vector[1], three = num2, four = num3);
}
pub fn reverse_polish_notation(){
	println!("This is the rpn calculator, it is not finished yet");
}
pub fn help(){
	println!("MathTools Help: \nFor help; type \"help\"\nFor simple arithmetic(multiplication, addition, subtraction, exponents, etc), type \"simple\"\nFor Reverse polish notation, type \"sausage\"\nTo exit, type \"exit\"");
}
