# Parker

Fastest way to get started with Rust and Rocket, where we have looked at the three criteria for getting something to production.

## Goals

## Reliability

Meaning that it works correctly, even when things go wrong. It's rust and its safety guarantees and error handling which is going to tick this box.

We will also setup unit tests and integrations tests and make sure that they run in a CI/CD pipeline.

## Scalability

Ability to handle load, where I demonstrate how to test performance for your application.

## Maintainability

Maintaibility falls under operability, simplicity and evolvability. To make this work we will setup Docker container, deploy the application under fly.io and monitor it through Prometheus and Grafana.

### Rustfmt configuration

We added a rustfmt configuration to make sure that code is consistent across developers. In the future we will also format imports and comments, but that's only available on Nightly right now.

## Software

| Name     | Use                                             |
|----------|-------------------------------------------------|
| [Rocket] | Easy to use and fast web framework              |
| [SQLx]   | Write plain SQL, but with compile-time checking |

[Rocket]: https://rocket.rs
[SQLx]: https://github.com/launchbadge/sqlx

### SQLx

We use SQLx for migrations and for running queries against our database. To configure your PostgreSQL instance -- running on Docker -- you have to install [direnv] and configure it by copying `direnv.example` to `.direnv`.

Then we do the following steps to get the database ready:

1. Start up the database with `make postgres`
2. Make sure the database exists with `sqlx create`
3. Run all the migrations with `sqlx migrate run`

[direnv]: https://github.com/direnv/direnv

## Credits

I followed along the excellent [Zero To Production In Rust] book and made adaptations where I saw fit. Here's where I deviate from the book:

* More reliance on [Makefile] instead of scripts.
* Use the [tide] web framework instead of Actix web. I found the API to be cleaner.

[Zero To Production In Rust]: https://www.zero2prod.com/index.html?country=Netherlands&discount_code=VAT20
[Makefile]: ./Makefile
[tide]: https://github.com/http-rs/tide
