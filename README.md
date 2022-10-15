# slower
Print STDOUT slower!

# Installation 
For convenience you can use `install.sh`. This will compile the program and copy the executable to /usr/bin/slower

...Or just exeucute the included binary.

# Usage
Pipe any command into slower to print its standard output slower
```
neofetch | slower
```

You can also specify a timeout (µs) in the first argument. The default timeout is 1000µs.
```
neofetch | slower 500
```
# About 
This is my first rust program so I decided to rewrite my [slow](https://github.com/eriknj99/slow) project which was my first C program.







