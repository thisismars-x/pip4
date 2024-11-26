
# pip4.0

pip4 does one simple thing and does it good.

It manufactures a requirements.txt file for your python project, skipping any dependency(direct or transient) that you do not directly invoke in your .py file.

pip4 is written completely in Rust. So make sure to have it on your system. [Here is an awesome tutorial](https://www.rust-lang.org/tools/install)


## Installation

Just clone this repo and then navigate to your Cargo.toml

```bash
  cargo install --path .
```
    
## Example

If you are in a directory X, and want a requirements file for directory Y.

```bash 
pip4 Y
```

This generates a requirements.txt file.

Omitting Y would force to work on the same directory.

