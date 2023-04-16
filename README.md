# Web API sample program

## TLS

The implemented web server is certified with `openssl`. You need `key.pem` and `cert.pem` file in the root directory. To create a self-signed temporary cert for testing:

```bash
openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'
```

When testing with self-signed cert, you need `--insecure` option to execute `curl` command.

```bash
curl --insecure https://localhost:8080
```
