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

```
list-to-tabs --help
```
>Convert CLI list of hostnames to Konsole tab-from-file list
>
>Usage: list-to-tabs --list [<LIST>...]
>
>Options:
>  -l, --list [<LIST>...]  The pattern to look for
>  -h, --help              Print help

---

```bash
list-to-tabs --list host_name1 host_name2 host_name3
```
```bash
cat /tmp/output_tabs.txt
```
>Outputs tab file to /tmp/output_tabs.txt
>
>Contents of file are in the format of:\
>title: host_name1;; command: ssh host_name1\
>title: host_name2;; command: ssh host_name2\
>title: host_name3;; command: ssh host_name3

```bash
konsole --tabs-from-file /tmp/output_tabs.txt
```




## License

[Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
