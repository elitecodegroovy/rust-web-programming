struct SharedResource {
    id: u32,
}

impl SharedResource {
    fn new(id: u32) -> Self {
        println!("Creating SharedResource with ID: {}", id);
        SharedResource { id }
    }
}

impl Drop for SharedResource {
    fn drop(&mut self) {
        println!("Dropping SharedResource with ID: {}", self.id);
    }
}

fn main() {
    use std::rc::Rc;
    let resource = Rc::new(SharedResource::new(1000));
    println!("Resource count: {}", Rc::strong_count(&resource));

    {
        let resource_clone = Rc::clone(&resource);
        println!("Resource count after clone: {}", Rc::strong_count(&resource));
    } // `resource_clone` is dropped here

    println!("Resource count after inner scope: {}", Rc::strong_count(&resource));
} // `resource` is dropped here