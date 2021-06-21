pub struct Foo {
    num: i32
}

impl Foo {
    pub fn set(&mut self, value: i32) {
        self.num = value;
    }

    pub fn get(&self) -> i32 {
        return self.num;
    }

    pub fn new() -> Foo {
        return Foo {
            num: 0
        }
    }
}
