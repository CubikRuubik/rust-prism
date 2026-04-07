use colored::*;

fn print_colored(label: &str, message: &str) {
    print!("[{}] ", label.green());
    println!("{}", message.yellow());
}

fn main() {
    println!("Hello, World!");
    println!("First Commit");
    println!("Second Commit");
    print_colored("info", "colored output via fatih/color");
}
