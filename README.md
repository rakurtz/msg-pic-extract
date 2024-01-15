
# msg-extractor

A simple to use, cross-platform .msg-file attachment extractor written in Rust.


## Acknowledgements
msg-extractor is mainly based on the great crate msg-parser by marirs. Thanks, man!
 - [msg-parser](https://github.com/marirs/msg-parser-rs)
 - [Open](https://github.com/Byron/open-rs)
 - [Slint](https://github.com/slint-ui/slint/blob/master/LICENSES/LicenseRef-Slint-Royalty-free-1.1.md)


## Installation
### Building from source
Just clone the repository and compile the binary yourself:

```
git clone https://github.com/rakurtz/msg-pic-extract.git
cd msg-pic-extract
cargo check
cargo build --release 
```    
After a successful build you'll find the binary in the `target/release` folder.


### Download a binary
[Releases](https://github.com/rakurtz/msg-pic-extract/releases/)

Windows: [msg-extractor.exe](https://github.com/rakurtz/msg-pic-extract/releases/download/v.01/msg-extractor.exe) v.0.1 (18.9 MB)

Mac: [msg-extractor](https://github.com/rakurtz/msg-pic-extract/releases/download/v.01/msg-extractor) v.0.1 (7.5 MB)


## Screenshots

![App Screenshot](https://github.com/rakurtz/msg-pic-extract/blob/main/screenshot.png)


## Usage/Examples

1. execute msg-extractor.exe or run the binary on mac
2. click on open temporary folder
3. copy & paste or drag & drop your .msg-files to the opened folder 
4. click on "Run" (Extrahieren!)

The program will create a new folder for each .msg-file, named with a short randomized prefix, sender's name and subject. Inside this folder you should find all attachments of that particular email and also the .msg file itself.

- Also extracts inline / embedded attachments
- works with multiple .msg files at once

5. Clean up removes all temporary files and exits msg-extractor.
## License

[MIT](https://choosealicense.com/licenses/mit/)

