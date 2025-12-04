

To create the key.pem and cert.pem use the command. Fill in your own subject

``` 
$ openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem \
-days 365 -sha256 -subj "/C=CN/ST=Guangdong/L=Guangzhou/O=TVlinux/OU=Org/CN=fastly.org"

```
To remove the password, then copy nopass.pem to key.pem

``` 
$ openssl rsa -in key.pem -out nopass.pem
```

