# Web API sample program

## TLS

The implemented web server is certified with `openssl`. To create a self-signed temporary cert for testing:

```bash
openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'
```
