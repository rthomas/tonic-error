## Example usage of `TonicError`

This is an example project for using `TonicError` with Tonic, there is a shared
`common` module that contains the protos and our error type, that derives `TonicError`.

To run this, start the server, and then run the client in another terminal.

### Start the server

```
$ cargo run --bin server
```

### Run the client

```
$ cargo run --bin client
```