# Actix Starter Web App

An example structure to develop a simple backend web application with Actix.

## Getting Started

```bash
git clone git@github.com:mcansahin/actix-starter-web-app.git
cd actix-starter-web-app
# create database
cp .env.example .env
# set database info. in .env
cargo install diesel_cli
diesel migration run
cargo install systemfd cargo-watch
systemfd --no-pid -s http::3000 -- cargo watch -x run
```

If you run into an error like:

```
note: ld: library not found for -lmysqlclient
clang: error: linker command failed with exit code 1 (use -v to see invocation)
````

This means you are missing the client library needed for a database backend – mysqlclient in this case. You can resolve this issue by either installing the library (using the usual way to do this depending on your operating system) or by specifying the backends you want to install the CLI tool with. [https://diesel.rs/guides/getting-started/](Diesel.rs)

## APIs

```
POST /api/auth/register
Accept-Encoding: application/json
Content-Type: application/json
Body: {"email":"email@test.com","password":"pass"}

POST /api/auth/login
Accept-Encoding: application/json
Content-Type: application/json
Body: {"email":"email@test.com","password":"pass"}

GET /api/auth/me
Authorization: Bearer ...
```