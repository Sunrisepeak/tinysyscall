# tiny syscall
a (no depend std) tiny syscall abstract layer...



## Usage

Add the following to your `Cargo.toml`:

```rust
[dependencies]
tinysyscall = "0.1.0"
```



## Examples

```rust
use tinysyscall;

fn main() {
    let hw = "Hello, World!\n";
    tinysyscall::file::write(tinysyscall::file::STDOUT, hw.as_bytes());
}
```



- [**more examples**](examples)



## Pub API

> 系统调用 接口

| 模块     | 接口 | 备注                 |
| ------- | ---------- | -------------------- |
| `file`  | `open  `     | :white_check_mark: |
|         | `read  `     | :white_check_mark: |
|         | `write `     | :white_check_mark: |
|         | `ioctl `     | :white_check_mark: |
|         | `stat  `     | :white_check_mark: |
|         | `close `     | :white_check_mark: |
|         |              |                |
| `mem`   | `mmap  `     | :white_check_mark: |
|         | `unmmap`     | :white_check_mark: |
|         |              |                |
| `time`  | `sleep `     |                |
|         |              |                |
| `process` | `exit  `     | :white_check_mark: |
|         |            |                      |
| `task`  |            | thread/task 暂不支持 |
|         |            |                      |



## OS & Arch Support

| 系统       | 架构      | 备注               |
| ---------- | --------- | ------------------ |
| `linux`    | `x8_64`   | :white_check_mark: |
|            | `riscv64` | :white_check_mark: |
|            |           |                    |
| `windows`  |           |                    |
|            |           |                    |
| `mac`      |           |                    |
|            |           |                    |
| `freertos` |           |                    |
|            |           |                    |



## Other

- [Crate Status](https://crates.io/crates/tinysyscall)

- [Linux](https://github.com/torvalds/linux)
  - [x86_64](https://x64.syscall.sh/)
  - [riscv64](https://jborza.com/post/2021-05-11-riscv-linux-syscalls)
