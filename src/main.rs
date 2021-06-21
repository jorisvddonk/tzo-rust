use std::fs;

mod tests;
mod vm;

fn testfn(vm: &mut vm::VM) {
    println!("\nTest foreign function called! PC: {}\n", vm.pc);
}

fn main() {
    let contents = fs::read_to_string("test.json").expect("Something went wrong reading the file");
    let v: serde_json::Value = serde_json::from_str(&contents).unwrap();
    let instructions = v.as_array().unwrap().to_vec();

    let mut vm = vm::VM::new();
    vm.registerForeignFunction("test".to_string(), testfn);
    vm.load(instructions);
    vm.run();

    println!("\nStack at end: {:?}", vm.stack);
}
