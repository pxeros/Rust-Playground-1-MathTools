mod functions;
fn main() {
	loop {
		println!("Welcome to MathTools, Please ask me what you want me to do?");
		let entry = functions::get_input().to_lowercase();
		if entry.trim() == "test" {
			functions::do_thing();
			println!("If it appeared, we are a happy people.");
		}else if entry.trim() == "simple" {
			functions::simple_arithmetic();
		}else if entry.trim() == "sausage" {
			functions::reverse_polish_notation();
		}else if entry.trim() == "help" {
			functions::help();
		}else if entry.trim() == "exit" {
			break;
		}else{
			println!("not a valid command, type help for more help");
		}
	}
	println!("Goodbye:");
}
