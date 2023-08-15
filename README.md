# RIND

Find implementation in Rust for fun

## RDD

Rind supports the basic functionality of find. It can traverse a filesystem searching for files and directories using a small number of search criteria.

To start, Rind will support the following behavior:
- Searching for files in a given directory using the flag `--path`
- Searching for files with a given extensions using the flag `--ext`
- Searching for files larger than a specific size using the flag `--size-gt`
- Searching for files that are _newer_ than a given timestamp `--time-gt`
