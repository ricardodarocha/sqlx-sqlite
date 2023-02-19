
<h1 align="center">
  <br>
  <a href="http://www.ricardodarocha.com.br"><img src="https://styles.redditmedia.com/t5_2s7lj/styles/communityIcon_pjg3ktzyju771.png" alt="Rust" width="200"></a>
  <br>
  Rust
  <br>
</h1>

<h4 align="center">About  </p>
An example of using <a href="https://www.sqlite.org/index.html" target="_blank">SQLite</a> with <a href="https://docs.rs/sqlx/latest/sqlx" target="_blank">sqlx</a> , without any ORM, using just Native_tls async.</h4>

<p align="center">
  <a href="#introdução">Introduction</a> •
  <a href="#como-usar">How to use</a> •
  <a href="#download">Download</a> •
  <a href="#credits">Credits</a> •
  <a href="#related">Related</a> •
  <a href="#license">License</a>
</p>

![screenshot](https://i.ytimg.com/vi/VuVOyUbFSI0/maxresdefault.jpg)

## Introduction

If you want a light example you can try this.

**Example**
```rust
//async
let result = sqlx::query_as!(User, "SELECT * FROM users")
    .fetch_all(&mut connection).await;
    result.unwrap()
```

The data can be accessed as an iterator
**dados.json**
```rust
#[async_std::main]
async fn main()  {
    for user in get_all().await {
        print!("{:?}",user)
    }
}```

##a How to use

Clone this repositorio [Git](https://github.com/ricardodarocha/sqlx-sqlite.git) 
Run `cargo check cargo run`

```bash
# Clone this repository
$ git clone https://github.com/ricardodarocha/sqlx-sqlite.git

# Go into the repository
$ cd sqlx-sqlite

# Open with VSCode
$ code .

# install and run
$ cargo check
$ cargo run 
```

## Download

You can download sqliteStudio https://sqlitestudio.pl/  if want to customize the database

## Credits

This software uses the following open source packages:

- [sqlx](https://docs.rs/sqlx/latest/sqlx)
- [native-tls](https://crates.io/crates/native-tls)

## Related

[Rust](https://www.rust-lang.org/pt-BR) - A linguagem mais querida 🦀

## Contato

> Linkedin [ricardo-da-rocha-vitor](https://www.linkedin.com/in/ricardo-da-rocha-vitor-a0983932/)
> Site [ricardodarocha.com.br](https://www.ricardodarocha.com.br) &nbsp;&middot;&nbsp;
> GitHub [@ricardodarocha](https://github.com/ricardodarocha) &nbsp;&middot;&nbsp;
> Twitter [@ricardorochadev](https://twitter.com/ricardorochadev)


## You may also like...

- [Actix-Web](https://actix.rs/) 
- [Axum](https://docs.rs/axum/latest/axum/)
- [Tokio](https://github.com/tokio-rs)
- [Postgres](https://rust-lang-nursery.github.io/rust-cookbook/database/postgres.html)

## License

GNU


---


