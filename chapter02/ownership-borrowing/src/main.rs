// chart02/ownership-borrowing
struct VPNNetwork {
    name: String
}

fn connect_vpn(vpn: VPNNetwork) {
    println!(" >>> connect vpn network '{}'", vpn.name)
}

fn main() {
    println!("Ownership & Borrowing are considered by many to be some of the most challenging things!");

    let network = VPNNetwork{name: "Local network".to_string() };
    connect_vpn(network);
    // connect_vpn(network);
}
