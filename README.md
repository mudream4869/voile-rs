# Voile

Voile is a app designed for personal book reading that supports various file formats. It allows users to store their preferred book format, be it a folder of images or texts, for easy reading.

## Target

### Will Provide

* Support books format:
    * A folder of images
    * A folder of txts
    * One txt (Maybe large)
    * pdf
    * epub
* Operations:
    * Upload/Remove books
    * Edit book information
    * Group books into a series
* Single-user login with password authentication.
* Single binary executable.
* A self-hosted server.

### **May (or may not 😝)** Provide

* Offers WebDAV as an alternative source for books.

### Won't Provide

* Multi-user
* Crawler
* Native GUI

## How to build and run

(WIP)

Since the GUI is provided by web page, we should build frontend first.

### Build frontend

```bash
cd frontend
yarn build
```

### Build backend

```bash
cargo build
```

### Start

```bash
cargo run configs/voile.toml
```

## TODO

- [ ] Easy configuration
- [x] Use rust-embed to embed prebuild frontend
- [ ] Search
- [x] Tag Filter
- [x] Add zip
- [ ] Upload book cover
- [x] PDF Book
- [ ] i18n

## License

MIT
