# mfo-api

![Tests](https://github.com/brokenthorn/mfo-api/workflows/Main/badge.svg)

`mfo-api` is a small and fast backend [Web API](https://en.wikipedia.org/wiki/Web_API) server written in Rust and used by the [minifarmonline.ro](https://minifarmonline.ro) store to get information (eg. products, prices, stock) to and from the point of sale applications used in the physical locations where purchase orders are processed and from where products are shipped to customers.

A Dockerfile exists so it can easily be deployed to Docker or public clouds like AWS or Azure.

## Build

Install [Rust](https://www.rust-lang.org/tools/install) and run `cargo build --release` inside the project folder:

```bash
$ cargo build --release
    Compiling mfo-api v0.1.0 (/.../mfo-api)
    # output omitted
    Finished release [optimized] target(s) in 19.72s
```

## Run

You can run the server for manual testing by just running:

```bash
$ HOST=0.0.0.0 PORT=8080 MSSQL_HOST=10.0.0.1 MSSQL_PORT=1433 MSSQL_USER=sql_user MSSQL_PASSWORD=sql_password cargo run
{"level":30,"time":1596533478892,"msg":"Logger started","level":INFO}
{"level":30,"time":1596533478892,"msg":"Application starting up."}
# output omitted
```

Don't forget to set `MSSQL_*` environment variables before starting the server, otherwise the server will shutdown the first time it tries to connect to the MSSQL server and fails to read these variables.

`HOST` and `PORT` specify the host and port number that the web server will listen on, and are optional. Their defaults are the same as above.

The server can also be run from the cargo build directory by running the latest built static binary:

```bash
# check the latest built binary:

$ ls -lh target/release/mfo-api*
-rwxr-xr-x  2 user  staff   4.9M Aug  4 23:19 target/release/mfo-api
-rw-r--r--  1 user  staff   573B Aug  4 23:19 target/release/mfo-api.d

# don't forget to set required environment variables first, then run:

$ ./target/release/mfo-api
{"level":30,"time":1596533478892,"msg":"Logger started","level":INFO}
{"level":30,"time":1596533478892,"msg":"Application starting up."}
# output omitted
```

## Build Docker image

Change directory into the project folder and run:

```bash
# this will build a `mfo-api:latest` image
$ docker build -t mfo-api .

# now start a container using this image:
$ docker run --env MSSQL_HOST=10.0.0.1 --env MSSQL_PORT=1433 --env MSSQL_USER=sql_user --env MSSQL_PASSWORD=sql_password -p 8080:8080 mfo-api:latest
```

You can add a `-d` argument after `docker run`, to detach the console from the container, so you won't lock the current console, or you can press `Ctrl+C` to detach after you started the container without the `-d` argument.

You can always follow server logs with `docker logs --follow [CONTAINER ID]`, where the container ID can be found with `docker ps -q --filter "ancestor=mfo-api"`.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
