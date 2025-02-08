# Simple Key-Value Storage in Rust

This is a minimal key-value storage application built with Rust using the `actix-web` framework. The application provides an HTTP-based interface for storing and retrieving values. Data is persisted in a file named `kv`.

## Features
- RESTful API using `actix-web`
- Simple key-value storage
- File-based persistence
- No external configuration (port and file details are defined in `main.rs`)


By default, the application runs on `http://localhost:8080` and stores data in the `kv` file.


## Future Improvements
Feel free to clone, enhance, and add new features! Here are some ideas:
- Add a configuration structure for better flexibility
- Implement an in-memory cache for faster lookups
- Support different storage backends (e.g., SQLite, RocksDB)
- Improve error handling and logging

### Contributing
Pull requests are welcome! If you find any issues or have feature requests, feel free to open an issue.

## License
MIT License

