// chart02/ownership-borrowing2
struct VPNNetwork {
    name: String,
    connect_count: i32
}

fn connect_vpn(vpn: &mut VPNNetwork) {           // ① it indicates that vpn can be modified
    println!(" >>> connect vpn network '{}', connect number {}", vpn.name, vpn.connect_count);
    vpn.connect_count += 1;                      // ② connect_count value is changed
}

fn main() {
    println!("Ownership & Borrowing are considered by many to be some of the most challenging things!");

    let mut network = VPNNetwork{name: "Local network".to_string(),  // ③ mutable variable structure instance
        connect_count: 0
    };
    connect_vpn(&mut network);                   // ④ The expression to create a mutable
                                                 // reference also requires the addition of the mut keyword
    connect_vpn(&mut network);
}
