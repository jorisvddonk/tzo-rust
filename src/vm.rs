use rand::{thread_rng, Rng};
use serde_json;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::Hasher;

pub struct VM {
    pub pc: usize,
    pub stack: Vec<Value>,
    pub programlist: Vec<Instr>,
    pub context: HashMap<u64, Value>,
    pub labels: HashMap<String, i64>,
    pub foreign_functions: Vec<ForeignFunc>,
    pub running: bool,
    pub exited: bool,
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
}

pub enum Instr {
    Number(f64),
    String(String),
    Func(Func),
    OpenBrace,
    CloseBrace,
}

pub struct ForeignFunc {
    pub func: fn(&mut VM),
    pub name: String,
}

type Func = fn(&mut VM);

impl Value {
    pub fn is_string(&self) -> bool {
        match self {
            Value::Number(_a) => false,
            Value::String(_a) => true,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Value::Number(_a) => true,
            Value::String(_a) => false,
        }
    }

    pub fn as_number(&self) -> f64 {
        match self {
            Value::Number(a) => *a,
            Value::String(_a) => panic!("as_number: Value is not a number"),
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Value::Number(_a) => panic!("as_string: Value is not a String"),
            Value::String(a) => a.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::Number(a) => format!("{}", a),
            Value::String(a) => a.to_string(),
        }
    }

    pub fn as_hash(self) -> u64 {
        let mut hasher = DefaultHasher::new();
        match self {
            Value::Number(a) => hasher.write_i64(a as i64),
            Value::String(a) => hasher.write(a.as_bytes()),
        }
        return hasher.finish();
    }
}

impl VM {
    pub fn put(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn put_f64(&mut self, value: f64) {
        self.stack.push(Value::Number(value));
    }

    pub fn put_string(&mut self, value: String) {
        self.stack.push(Value::String(value));
    }

    pub fn i_plus(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        if a.clone().is_number() && b.clone().is_number() {
            self.stack
                .push(Value::Number(a.as_number() + b.as_number()));
        }
    }

    pub fn i_nop(&mut self) {}

    pub fn i_closebrace(&mut self) {}
    pub fn i_openbrace(&mut self) {
        let mut i = 1;
        let mut ppc = self.pc + 1;
        while ppc < self.programlist.len() {
            let v = self.programlist.get(ppc).unwrap();
            match v {
                Instr::Number(_) => {}
                Instr::String(_) => {}
                Instr::Func(_) => {}
                Instr::OpenBrace => {
                    i += 1;
                }
                Instr::CloseBrace => {
                    i -= 1;
                    if i == 0 {
                        // found it!
                        self.pc = ppc; // will be incremented later!
                        return;
                    }
                }
            }
            ppc += 1;
        }
        panic!("No matching brace found!");
    }

    pub fn i_pop(&mut self) {
        self.stack.pop();
    }

    pub fn i_min(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        if a.clone().is_number() && b.clone().is_number() {
            self.stack
                .push(Value::Number(a.as_number() - b.as_number()));
        }
    }

    pub fn i_mul(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        if a.clone().is_number() && b.clone().is_number() {
            self.stack
                .push(Value::Number(a.as_number() * b.as_number()));
        }
    }

    pub fn i_stdout(&mut self) {
        let a = self.stack.pop().unwrap();
        match a {
            Value::Number(x) => {
                print!("{}", x);
            }
            Value::String(x) => {
                print!("{}", x);
            }
        }
    }

    pub fn i_goto(&mut self) {
        let a = self.stack.pop().unwrap();
        if a.is_number() {
            self.pc = (a.as_number() - 1 as f64) as usize; // will be incremented after step!
        } else if a.is_string() {
            self.pc = (self.labels.get(&a.as_string()).unwrap() - 1) as usize;
            // will be incremented after step!
        }
    }

    pub fn i_concat(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack
            .push(Value::String(format!("{}{}", a.to_string(), b.to_string())))
    }

    pub fn i_rconcat(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack
            .push(Value::String(format!("{}{}", b.to_string(), a.to_string())))
    }

    pub fn i_randint(&mut self) {
        let a = self.stack.pop().unwrap().as_number() as i32;
        self.stack
            .push(Value::Number(thread_rng().gen_range(0..a) as f64));
    }

    pub fn i_charcode(&mut self) {
        let a = self.stack.pop().unwrap().as_number() as u8;
        self.stack.push(Value::String(format!("{}", a as char)));
    }

    pub fn i_not(&mut self) {
        let a = self.stack.pop().unwrap().as_number() as i32;
        if a == 0 {
            self.stack.push(Value::Number(1 as f64));
        } else {
            self.stack.push(Value::Number(0 as f64));
        }
    }

    pub fn i_or(&mut self) {
        let a = self.stack.pop().unwrap().as_number() as i32;
        let b = self.stack.pop().unwrap().as_number() as i32;
        if a == 0 && b == 0 {
            self.stack.push(Value::Number(0 as f64));
        } else {
            self.stack.push(Value::Number(1 as f64));
        }
    }

    pub fn i_and(&mut self) {
        let a = self.stack.pop().unwrap().as_number() as i32;
        let b = self.stack.pop().unwrap().as_number() as i32;
        if a == 0 || b == 0 {
            self.stack.push(Value::Number(0 as f64));
        } else {
            self.stack.push(Value::Number(1 as f64));
        }
    }

    pub fn i_jgz(&mut self) {
        let a = self.stack.pop().unwrap().as_number() as i32;
        if a > 0 {
            self.pc += 1;
        }
    }

    pub fn i_jz(&mut self) {
        let a = self.stack.pop().unwrap().as_number() as i32;
        if a == 0 {
            self.pc += 1;
        }
    }

    pub fn i_gt(&mut self) {
        let a = self.stack.pop().unwrap().as_number() as i32;
        let b = self.stack.pop().unwrap().as_number() as i32;
        if a > b {
            self.stack.push(Value::Number(1 as f64));
        } else {
            self.stack.push(Value::Number(0 as f64));
        }
    }

    pub fn i_lt(&mut self) {
        let a = self.stack.pop().unwrap().as_number() as i32;
        let b = self.stack.pop().unwrap().as_number() as i32;
        if a < b {
            self.stack.push(Value::Number(1 as f64));
        } else {
            self.stack.push(Value::Number(0 as f64));
        }
    }

    pub fn i_dup(&mut self) {
        let a = self.stack.pop().unwrap();
        match a {
            Value::Number(x) => {
                self.stack.push(Value::Number(x));
                self.stack.push(Value::Number(x));
            }
            Value::String(x) => {
                self.stack.push(Value::String(x.clone()));
                self.stack.push(Value::String(x.clone()));
            }
        }
    }

    pub fn i_eq(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        match a {
            Value::Number(_) => {
                if b.clone().is_number() {
                    if b.as_number() == a.as_number() {
                        self.stack.push(Value::Number(1 as f64));
                    } else {
                        self.stack.push(Value::Number(0 as f64));
                    }
                } else {
                    self.stack.push(Value::Number(0 as f64));
                }
            }
            Value::String(_) => {
                if b.clone().is_string() {
                    if a.as_string().eq(&b.as_string()) {
                        self.stack.push(Value::Number(1 as f64));
                    } else {
                        self.stack.push(Value::Number(0 as f64));
                    }
                } else {
                    self.stack.push(Value::Number(0 as f64));
                }
            }
        }
    }

    pub fn i_ppc(&mut self) {
        self.stack.push(Value::Number(self.pc as f64));
    }

    pub fn i_stacksize(&mut self) {
        self.stack.push(Value::Number(self.stack.len() as f64));
    }

    pub fn i_pause(&mut self) {
        self.running = false;
    }

    pub fn i_exit(&mut self) {
        self.running = false;
        self.exited = true;
    }

    pub fn i_getcontext(&mut self) {
        let a = self.stack.pop().unwrap();
        let hash = &a.as_hash();
        let r = self.context.get(hash).unwrap();
        match r {
            Value::Number(x) => {
                self.stack.push(Value::Number(*x as f64));
            }
            Value::String(x) => {
                self.stack.push(Value::String(x.clone()));
            }
        }
    }
    pub fn i_hascontext(&mut self) {
        let a = self.stack.pop().unwrap().as_hash();
        if self.context.contains_key(&a) {
            self.stack.push(Value::Number(1 as f64));
        } else {
            self.stack.push(Value::Number(0 as f64));
        }
    }
    pub fn i_delcontext(&mut self) {
        let a = self.stack.pop().unwrap().as_hash();
        if self.context.contains_key(&a) {
            self.context.remove(&a);
        } else {
            // do nothing
        }
    }
    pub fn i_setcontext(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        let hash = a.as_hash();
        self.context.insert(hash, b);
    }

    pub fn new() -> VM {
        return VM {
            pc: 0,
            stack: std::vec::Vec::new(),
            programlist: std::vec::Vec::new(),
            running: false,
            exited: false,
            context: HashMap::new(),
            labels: HashMap::new(),
            foreign_functions: std::vec::Vec::new(),
        };
    }

    pub fn run(&mut self) {
        if !self.exited {
            self.running = true;
            while self.running && self.pc < self.programlist.len() {
                self.step();
            }
        } else {
            panic!("Program has already exited!");
        }
    }

    pub fn step(&mut self) {
        let z = self.programlist.get(self.pc);
        let z = match z {
            Some(i) => i,
            None => {
                panic!("Program counter exceeds program list size!");
            }
        };
        match z {
            Instr::Number(a) => {
                self.stack.push(Value::Number(*a));
            }
            Instr::String(a) => {
                self.stack.push(Value::String(a.clone()));
            }
            Instr::Func(a) => {
                a(self);
            }
            Instr::OpenBrace => {
                self.i_openbrace();
            }
            Instr::CloseBrace => {
                self.i_closebrace();
            }
        }
        self.pc += 1;
    }

    pub fn registerForeignFunction(&mut self, ffunc: ForeignFunc) {
        self.foreign_functions.push(ffunc);
    }

    pub fn load(&mut self, instructions: std::vec::Vec<serde_json::Value>) {
        for i in instructions.iter() {
            if i["type"] == "push-number-instruction" {
                self.programlist
                    .push(Instr::Number(i.get("value").unwrap().as_f64().unwrap()));
            }
            if i["type"] == "push-string-instruction" {
                self.programlist.push(Instr::String(String::from(
                    i.get("value").unwrap().as_str().unwrap(),
                )));
            }
            if i["type"] == "invoke-function-instruction" {
                if i["functionName"] == "nop" {
                    self.programlist.push(Instr::Func(VM::i_nop));
                } else if i["functionName"] == "pop" {
                    self.programlist.push(Instr::Func(VM::i_pop));
                } else if i["functionName"] == "plus" || i["functionName"] == "+" {
                    self.programlist.push(Instr::Func(VM::i_plus));
                } else if i["functionName"] == "min" || i["functionName"] == "-" {
                    self.programlist.push(Instr::Func(VM::i_min));
                } else if i["functionName"] == "mul" || i["functionName"] == "*" {
                    self.programlist.push(Instr::Func(VM::i_mul));
                } else if i["functionName"] == "concat" {
                    self.programlist.push(Instr::Func(VM::i_concat));
                } else if i["functionName"] == "rconcat" {
                    self.programlist.push(Instr::Func(VM::i_rconcat));
                } else if i["functionName"] == "randInt" {
                    self.programlist.push(Instr::Func(VM::i_randint));
                } else if i["functionName"] == "charCode" {
                    self.programlist.push(Instr::Func(VM::i_charcode));
                } else if i["functionName"] == "ppc" {
                    self.programlist.push(Instr::Func(VM::i_ppc));
                } else if i["functionName"] == "eq" {
                    self.programlist.push(Instr::Func(VM::i_eq));
                } else if i["functionName"] == "not" {
                    self.programlist.push(Instr::Func(VM::i_not));
                } else if i["functionName"] == "or" {
                    self.programlist.push(Instr::Func(VM::i_or));
                } else if i["functionName"] == "and" {
                    self.programlist.push(Instr::Func(VM::i_and));
                } else if i["functionName"] == "jgz" {
                    self.programlist.push(Instr::Func(VM::i_jgz));
                } else if i["functionName"] == "jz" {
                    self.programlist.push(Instr::Func(VM::i_jz));
                } else if i["functionName"] == "gt" {
                    self.programlist.push(Instr::Func(VM::i_gt));
                } else if i["functionName"] == "lt" {
                    self.programlist.push(Instr::Func(VM::i_lt));
                } else if i["functionName"] == "dup" {
                    self.programlist.push(Instr::Func(VM::i_dup));
                } else if i["functionName"] == "pause" {
                    self.programlist.push(Instr::Func(VM::i_pause));
                } else if i["functionName"] == "exit" {
                    self.programlist.push(Instr::Func(VM::i_exit));
                } else if i["functionName"] == "goto" {
                    self.programlist.push(Instr::Func(VM::i_goto));
                } else if i["functionName"] == "{" {
                    self.programlist.push(Instr::OpenBrace);
                } else if i["functionName"] == "}" {
                    self.programlist.push(Instr::CloseBrace);
                } else if i["functionName"] == "getContext" {
                    self.programlist.push(Instr::Func(VM::i_getcontext));
                } else if i["functionName"] == "hasContext" {
                    self.programlist.push(Instr::Func(VM::i_hascontext));
                } else if i["functionName"] == "setContext" {
                    self.programlist.push(Instr::Func(VM::i_setcontext));
                } else if i["functionName"] == "delContext" {
                    self.programlist.push(Instr::Func(VM::i_delcontext));
                } else if i["functionName"] == "stacksize" {
                    self.programlist.push(Instr::Func(VM::i_stacksize));
                } else if i["functionName"] == "stdout" {
                    self.programlist.push(Instr::Func(VM::i_stdout));
                } else {
                    let fname = i["functionName"].as_str().unwrap();
                    let mut found = false;
                    for i in &self.foreign_functions {
                        if i.name.eq(fname) {
                            self.programlist.push(Instr::Func(i.func));
                            found = true;
                        }
                    }
                    if found {
                    } else {
                        if !fname.starts_with("_") {
                            panic!("Function not found: {}", fname);
                        }
                    }
                }
            }
            if i.as_object().unwrap().contains_key("label") {
                let k = i.get("label").unwrap().as_str().unwrap().to_string();
                self.labels.insert(k, (self.programlist.len() - 1) as i64);
                print!("> {:?}", self.labels);
            }
        }
    }
}
