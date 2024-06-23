# shofi

shofi is a Rust program that concurrently reads and displays the contents of files matching specified glob patterns.

## Features

- Support for multiple glob patterns
- Concurrent file processing
- Formatted display of filenames and their contents

## Quick Installation

You can use the following one-liner command to clone the repository, build the project, and install the binary:

```bash
git clone https://github.com/ie-Yoshisaur/shofil.git && cd shofil && cargo build --release && sudo mv target/release/shofi /usr/local/bin/
```

This command does the following:
1. Clones the repository
2. Changes into the project directory
3. Builds the project in release mode
4. Moves the binary to /usr/local/bin/ (which is typically in your PATH)

Note: This command uses `sudo` to move the binary to /usr/local/bin/. You'll be prompted for your password.

If you prefer not to use sudo or want to install to a different location, you can modify the last part of the command. For example, to install to ~/bin/:

```bash
git clone https://github.com/ie-Yoshisaur/shofil.git && cd shofil && cargo build --release && mkdir -p ~/bin && mv target/release/shofi ~/bin/ && echo 'export PATH="$HOME/bin:$PATH"' >> ~/.bashrc && source ~/.bashrc
```

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
