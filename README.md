# Tutorial 3 - YewChat

## Cara Menjalankan

1. Jalankan WebSocket server:

```bash
cd SimpleWebsocketServer
npm i
npm start
```

Server default jalan di `ws://localhost:8080`.

2. Di terminal lain, jalankan YewChat:

```bash
cd YewChat
npm i
npm start
```

Aplikasi default dibuka di `http://localhost:8000`.

## Yang Saya Coba (Play With It)

1. Menjalankan server dan memastikan terminal menampilkan server aktif.
2. Menjalankan frontend YewChat di browser.
3. Mencoba koneksi ke WebSocket server.
4. mencoba kirim pesan antar 2 tab browser.
5. Mengamati update daftar user aktif saat tab dibuka/ditutup.

## Hasil Pengamatan Singkat

- Arsitekturnya dipisah: backend WebSocket sendiri, frontend Yew sendiri.
- Komunikasi real-time berjalan lewat event WebSocket (`register`, `message`, dan update user list).

### Terminal Server dan Yewchat

![Server and Yewchat Running](pics/terminal.png)

### Landing page

![landing](pics/landing.png)

### Demo Chat

![Demo Chat](pics/chat-room.png)
