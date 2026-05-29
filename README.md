# Actix Web + Mailexam

Minimal [Actix Web](https://actix.rs/) example that sends test email through [Mailexam](https://mailexam.ru/) SMTP.

This repository is the reference implementation for the [Actix Web integration guide](https://github.com/mailexam/wiki/blob/main/docs/en/examples/actix.md).

## Requirements

- Rust 1.70+
- A Mailexam account with SMTP credentials for a project

## Setup

1. Copy the example environment file and fill in your Mailexam credentials:

```bash
cp .env.example .env
```

2. Set values from your Mailexam welcome email or dashboard:

| Variable | Description |
|----------|-------------|
| `MAILEXAM_LOGIN` | SMTP login (host becomes `{login}.mailexam.ru`) |
| `MAILEXAM_PASSWORD` | SMTP password (paired with the login) |
| `MAILEXAM_PORT` | SMTP port (default `587`, STARTTLS) |
| `MAIL_FROM` | Sender address (any test address is fine) |

Do not commit `.env` to git.

## Run

```bash
cargo run
```

The server listens on `http://127.0.0.1:8080`.

## Send a test message

```bash
curl -X POST http://127.0.0.1:8080/mail/test \
  -H 'Content-Type: application/json' \
  -d '{"to":"user@example.test","subject":"Test","body":"Hello"}'
```

On success the endpoint returns `ok`. The message appears in the Mailexam dashboard under your project inbox.

## Project layout

```
src/
  main.rs   # HTTP server and POST /mail/test handler
  mail.rs   # Mailexam SMTP config and send_test via lettre
```

## See also

- [Mailexam wiki — Actix Web guide](https://wiki.mailexam.ru/en/examples/actix/)
- [Axum reference implementation](https://github.com/mailexam/Axum) — same mail module, different HTTP framework
- [Mailexam API documentation](https://mailexam.ru/api)
