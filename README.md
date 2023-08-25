<div align="center">
<em><img src="bookshelf.png" height="90px"></em>
<h1>Voile<br/><sub>A personal book reading app.</sub></h1>
</div>

Voile is a app designed for personal book reading that supports various file formats. It allows users to store their preferred book format, be it a folder of images or texts, for easy reading.

<a href="https://www.flaticon.com/free-icons/bookshelf" title="bookshelf icons">Bookshelf icons created by Freepik - Flaticon</a>

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

## How to Run

To run the program, you need to specify a settings directory. If you start the program without any arguments, it will use the default folder depending on your operating system:

* Linux: `$HOME/.config/Voile/`
* Windows: `%APPDATA%\Voile\`
* macOS: `$HOME/Library/Application Support/Voile/`

To start the program with a specific path to the configuration folder, you can run the following command:

```bash
./app [config_dir]
```

Replace `[config_dir]` with the desired path to the folder containing the configuration files. This allows you to override the default settings directory and specify a custom location for the configuration files.

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
