# Konsole --tabs-from-file ssh list generator 
Rust CLI Utility for genetrating newline delimited file that contains command structures to be used by the "Konsole --tabs-from-file" feature

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
## Build Locally

```bash
git clone https://github.com/jackpots28/list_to_konsole-tabs_rs.git
```
```bash
cd list_to_konsole-tabs_rs
```
```bash
cargo build --release
```
```bash
cp -r target/release/list-to-tabs /usr/local/sbin/list-to-tabs
```

---

## Usage/Examples

```bash
list-to-tabs --help
```

```
Convert CLI list or newline file of hostnames to Konsole tab-from-file list

Usage: list-to-tabs [OPTIONS] <--list [<LIST>...]|--input-file <INPUT>>

Options:
  -l, --list [<LIST>...]    Space delimited list of hostnames
  -i, --input-file <INPUT>  Input file as newline delimited text file
  -o, --output-file <FILE>  The output path with filename [default: ./default_output.tabs]
  -h, --help                Print help

```

---

```bash
list-to-tabs -o sample_output_file.tabs --list host_name1 host_name2 host_name3
```
or
```bash
printf "host_name1\nhost_name2\nhost_name3\n" > server.list
list-to-tabs -o sample_output_file.tabs --input-file server.list
```


```bash
cat sample_output_file.tabs
```

>Outputs tab file to ./sample_output_file.tabs
>
>Contents of file are in the format of:\
>title: host_name1;; command: ssh host_name1\
>title: host_name2;; command: ssh host_name2\
>title: host_name3;; command: ssh host_name3

```bash
konsole --tabs-from-file sample_output_file.tabs
```




## License

[Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
