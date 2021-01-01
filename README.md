# Memory modify
This is just an example of how you can modify other programs memory in Windows

## Example
Start a process you'd like to modify memory on, ex. the provided example
```
cargo run --example modify-memory
```

Also run our main program in order to modify memory
```
cargo run
```

## Questions

### Can I write to a random memory without winapi?
No you cannot; not without being in ring0/acting as a kernel-driver(?) going outside the OS level

https://stackoverflow.com/questions/25599701/how-do-i-edit-random-memory