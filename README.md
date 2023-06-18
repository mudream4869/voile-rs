# Voile

Voile is a app designed for personal book reading that supports various file formats. It allows users to store their preferred book format, be it a folder of images or texts, for easy reading.

## Target

### Will Provide

* Support books format:
    * [x] A folder of images
    * [x] A folder of txts
    * [x] One txt (Maybe large)
    * [x] pdf
    * [ ] epub
* Operations:
    * [x] Upload/Remove books
    * [x] Edit book information
    * [ ] Group books into a series
* [ ] Single-user login with password authentication.
* [x] Single binary executable.

### **May (or may not 😝)** Provide

* Offers WebDAV as an alternative source for books.

### Won't Provide

* Multi-user
* Crawler
* Native GUI

## How to build

```bash
cargo build
```

## TODO

- [ ] Easy configuration
- [x] Use rust-embed to embed prebuild frontend
- [ ] Search
- [x] Tag Filter
- [x] Add zip
- [x] Upload book cover
- [x] PDF Book
- [ ] i18n

## License

MIT
