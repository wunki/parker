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

