# shofi

shofi is a Rust program that concurrently reads and displays the contents of files matching specified glob patterns.

## Features

- Support for multiple glob patterns
- Concurrent file processing
- Formatted display of filenames and their contents

## Usage

```
shofi <glob_pattern1> [glob_pattern2] ...
```

Example:
```
shofi *.txt src/*.rs
```

This example will display the contents of all .txt files in the current directory and all .rs files in the src directory.

## Dependencies

- glob: For file pattern matching
- rayon: For concurrent processing
- thiserror: For error handling

## Installation

1. Ensure you have Rust and cargo installed.
2. In the project directory, run:

```
cargo build --release
```

3. The executable will be generated at `target/release/shofi`.

## Error Handling

The program handles the following errors:

- I/O errors
- Glob pattern errors
- Glob matching errors
- Error when no glob patterns are provided

## Notes

- Be mindful of memory usage when processing a large number of files or very large files.
- Some files may not be readable depending on file access permissions.

## License

This project is released under the MIT License. See the LICENSE file for details.

## Contributing

Please use the GitHub Issue tracker for bug reports and feature requests. Pull requests are welcome.

## Author

Yoshisaur
