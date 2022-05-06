# Zero To Production Rust

[Zero To Production](https://www.zero2prod.com/index.html) by Luca Palmieri

## Core Concepts

### Actix Web

#### 🚀 HttpRequest

- Take care of all things related to a server (handle all **transport layer** concern)

#### 💻 App

- All general application server logic happens (**routing**, **middlewares**, **request handles**)
- Tak incoming Request and decide the Response output

#### 🚦 Route

- Take 2 parameters -> a **path** and a **route** which is an instance of Route struct
- Route combines handler with a sets of guards
- **Guard** is like the pattern matching but it is for Actix Web. If request pass the guard it can go to the handler

### Testing

- We use the **tests** folder to seperate between actual business logic and the test cases
- **tokio::spawn** takes a future server and run it on background without need to waiting for its completion

### Extractors

- As its name, extractors are used to extract certain things that come from the incoming request
- For example: Path, Query, JSON
- Can handle 10 extractors per handler function
