# Simplistic Heartbeat Tracker

A simple server to track active nodes of distributed services using Actix Web and Diesel Async (with Postgres).

## Prerequisite

* libpq (Needed for applying migrations synchronously)

For development:

* Cargo
* Diesel CLI (Used in the helper script `local/postgres`)
* Docker (Used in the helper script `local/postgres`)

## How to run

```sh
cargo build --release
local/postgres target/release/simplistic-heartbeat-tracker
```

For development:
```sh
local/postgres cargo run
```

## Usage

```console
[nix-shell:~]$ curl -HContent-Type:application/json -d'{"source":"A","duration":"10s"}' localhost:8080
{}
[nix-shell:~]$ curl -s localhost:8080 | jq
[
  {
    "id": "548328d1-186a-4584-86ea-a9138a87e9b7",
    "source": "A",
    "expiry": "2023-05-22T11:20:07.438098000Z"
  }
]

[nix-shell:~]$ sleep 11

[nix-shell:~]$ curl -HContent-Type:application/json -d'{"source":"B","duration":"10s"}' localhost:8080
{}
[nix-shell:~]$ curl -HContent-Type:application/json -d'{"source":"C","duration":"10s"}' localhost:8080
{}
[nix-shell:~]$ curl -HContent-Type:application/json -d'{"source":"D","duration":"10s"}' localhost:8080
{}
[nix-shell:~]$ curl -s localhost:8080 | jq
[
  {
    "id": "175e85bc-0e21-4258-8f04-cddb26c4d00e",
    "source": "B",
    "expiry": "2023-05-22T11:20:32.602960000Z"
  },
  {
    "id": "a6347c58-60be-47de-a575-d1d29eb9ff74",
    "source": "C",
    "expiry": "2023-05-22T11:20:35.777178000Z"
  },
  {
    "id": "bf6ef135-0487-4f54-acc3-e98f763ae14d",
    "source": "D",
    "expiry": "2023-05-22T11:20:39.107050000Z"
  }
]

```
