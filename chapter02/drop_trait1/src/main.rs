struct MyResource {
    name: String,
}

impl Drop for MyResource {
    fn drop(&mut self) {
        println!("Dropping resource: {}", self.name);
    }
}

fn main() {
    {
        #[allow(dead_code)]
        let resource = MyResource {
            name: String::from("My resource file"),
        };
        println!("My Resource Created");
    } // `resource` goes out of scope here, `drop` is called
    println!("Scope exited");
}