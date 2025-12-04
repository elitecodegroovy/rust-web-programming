

To create the key.pem and cert.pem use the command. Fill in your own subject

``` 
$ openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem \
-days 365 -sha256 -subj "/C=CN/ST=Guangdong/L=Guangzhou/O=TVlinux/OU=Org/CN=fastly.org"

```
To remove the password, then copy nopass.pem to key.pem

``` 
$ openssl rsa -in key.pem -out nopass.pem
```

In case you are working on a server project (e.g. hyper, iron, etc) that keeps running and you need it to be restarted when files change, you can use cargo watch. Install:

``` 
cargo install cargo-watch
```
And then run:

``` 
cargo watch -x run
```
And to watch changes in only the src folder and clear the console use:

``` 
cargo watch -c -w src -x run
```

See the cargo-watch README for more examples.