# Amoozeshyar encode suckless

https://github.com/nimaaskarian/amoozeshyar-encode-suckless/assets/88832088/2de7eb86-abbd-43f7-a251-16a6736bc0c8

A small program written in Rust. Reads your input from stdin and uses dark reverse engineering magic to convert them into searchable terms. Just like [Amoozshyar change](https://amozeshyar.info/change/) but faster.  
The binary is called `enc-am` which is an abbreviation for `encode Amoozeshyar`. Reads stdin like good old POSIX commands, And writes back to stdin.  
This is a tool created with POSIX in mind. Its better to be used with other tools like dmenu and a commandline clipboard manager.  

## Dependencies
- `rustc`
- `make`

## Install
Change the PREFIX variable inside `config.mk` to set the installation path.  

- For compiling:  
    ```
    make
    ```
- For (compiling and) installing:
    ```
    make install
    ```

## Uninstall
```
make uninstall
```
