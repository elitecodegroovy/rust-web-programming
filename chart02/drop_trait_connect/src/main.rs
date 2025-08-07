struct NetworkConnection {
    url: String,
    connected: bool,
}

impl NetworkConnection {
    fn new(url: &str) -> Self {
        println!("Connecting to {}", url);
        NetworkConnection {
            url: url.to_string(),
            connected: true,
        }
    }

    fn send(&self, data: &str) {
        if self.connected {
            println!("Sending data to {}: {}", self.url, data);
        }
    }
}

impl Drop for NetworkConnection {
    fn drop(&mut self) {
        if self.connected {
            println!("Disconnecting from {}", self.url);
            self.connected = false;
        }
    }
}

fn main() {
    {
        let conn = NetworkConnection::new("example.com");
        conn.send("Ping");
    } // `conn` is dropped here
    println!("Main function complete");
}