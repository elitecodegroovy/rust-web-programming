# testing


## test flag


Sometimes it can be beneficial to get full output streams from all tests by 
disabling capturing. We can do this by passing the --nocapture flag to the 
test binary. 

``` 
cargo test -- --nocapture
```

ams are muddied together
though, and this is because Rust runs tests in parallel by default. We can clean this up a bit
by running the tests only from a single thread. This is controlled via the --test-threads.

``` 
cargo test -- --nocapture --test-threads=1
```

test doc

``` 
cargo doc --open
```