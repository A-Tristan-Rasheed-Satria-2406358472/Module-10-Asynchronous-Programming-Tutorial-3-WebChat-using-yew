# Rust WebSocket Server

## Run

```bash
cargo run
```

Server listen di:

```text
ws://127.0.0.1:8080
```

## Protokol Message

- Register dari client:
  - `{ "messageType": "register", "data": "username", "dataArray": null }`
- Message dari client:
  - `{ "messageType": "message", "data": "isi pesan", "dataArray": null }`

Server broadcast:

- Users update:
  - `{ "messageType": "users", "dataArray": ["u1", "u2"], "data": null }`
- Message update:
  - `{ "messageType": "message", "data": "{\"from\":\"u1\",\"message\":\"hi\",\"time\":123}" }`
