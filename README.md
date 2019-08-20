# avgfile
Avgfile is a small and simple tool made in [Rust](https://www.rust-lang.org/) that calculates the average byte in a file.

### Installation
If you don't want to compile the thing yourself, download the latest release [here](https://github.com/Mnpn03/avgfile/releases).

If you instead want to compile avgfile, you can do so by getting [Rust](https://www.rust-lang.org/).
Once that is installed, clone the repository:
`git clone git@github.com:Mnpn03/avgfile.git` to clone with SSH, or
`git clone https://github.com/Mnpn03/avgfile.git` to clone with HTTPS.
Then you simply build it by running `cargo build --release`.

### Usage
```
$ avgfile /path/to/desired.file
```
`$ avgfile -t ` - Print the total amount of bytes in a file.

`$ avgfile -s <byte>` - Search for a byte in a file.

`$ avgfile -u` - Print the number of unique bytes in a file.

To get more help, run
```
$ avgfile -h
```
### Contribution
To contribute to the project, simply create a [pull request](https://github.com/Mnpn03/avgfile/pulls) or an [issue](https://github.com/Mnpn03/avgfile/issues).

### License
Avgfile is FOSS that comes with no warranty. Read more about the license used [here](https://github.com/Mnpn03/avgfile/blob/master/LICENSE).
