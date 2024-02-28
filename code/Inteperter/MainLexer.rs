use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

struct Interpreter {
    functions: HashMap<String, Vec<String>>,
}

impl Interpreter {
    fn new() -> Interpreter {
        Interpreter {
            functions: HashMap::new(),
        }
    }

    fn parse(&mut self, code: &str) {
        let lines: Vec<&str> = code.split('\n').collect();
        for line in lines {
            if line.starts_with("FUNC:") {
                let parts: Vec<&str> = line.split(' ').collect();
                let func_name = parts[1].to_string();
                let func_body: Vec<String> = parts[2..].iter().map(|s| s.to_string()).collect();
                self.functions.insert(func_name, func_body);
            }
        }
    }

    fn call(&self, func_name: &str) {
        match self.functions.get(func_name) {
            Some(_) => println!("Calling function {}", func_name),
            None => println!("Function {} not found", func_name),
        }
    }

    fn load_file(&mut self, filename: &str) {
        let mut file = File::open(filename).expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read the file");
        self.parse(&contents);
    }
}

fn main() {
    let mut interpreter = Interpreter::new();
    interpreter.load_file("test.dsc");
    interpreter.call("MAIN");
}
