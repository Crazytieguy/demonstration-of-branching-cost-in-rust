# Demonstration of the optimizability of match statements in rust

This is based on a [video by Molly Rocket](https://www.youtube.com/watch?v=tD5NrevFtbU&t=1214s) where he shows how clean code can lead to poor performance. This is more or less a port of his demonstation regarding 'hiding internal implementation details' to rust.

The branching code is about 3.4x slower than the non-branching code.

## Code

### Branching Code:

```rust
match shape.shape_type {
    Square => 1.0 * shape.width * shape.height,
    Rectangle => 1.0 * shape.width * shape.height,
    Triangle => 0.5 * shape.width * shape.height,
    Circle => PI * shape.width * shape.height,
}
```

### Non Branching Code:

```rust
let coefficient = match shape.shape_type {
    Square => 1.0,
    Rectangle => 1.0,
    Triangle => 0.5,
    Circle => PI,
};
coefficient * shape.width * shape.height
```

## Running the benchmark

```bash
$ cargo bench
   Compiling shape v0.1.0 (/home/yoav/playground/rust/shape)
    Finished bench [optimized] target(s) in 2.10s
     Running unittests src/lib.rs (target/release/deps/shape-69fae79cbed89125)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/get_area.rs (target/release/deps/get_area-a41c4608bcfe561c)
Shape Area/Branching/100000
                        time:   [703.49 µs 718.96 µs 733.51 µs]
Shape Area/Non Branching/100000
                        time:   [212.15 µs 215.83 µs 219.85 µs]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) low mild
  1 (1.00%) high mild
  1 (1.00%) high severe
```

## Viewing the assembly

### Branching Code:


```bash
$ cargo asm shape::get_area_branching
shape::get_area_branching:
 movzx   eax, byte, ptr, [rdi, +, 8]
 lea     rcx, [rip, +, .LJTI0_0]
 movsxd  rax, dword, ptr, [rcx, +, 4*rax]
 add     rax, rcx
 jmp     rax
.LBB0_1:
 movss   xmm0, dword, ptr, [rdi]
 mulss   xmm0, dword, ptr, [rdi, +, 4]
 ret
.LBB0_2:
 movss   xmm0, dword, ptr, [rdi]
 mulss   xmm0, dword, ptr, [rip, +, .LCPI0_1]
 mulss   xmm0, dword, ptr, [rdi, +, 4]
 ret
.LBB0_3:
 movss   xmm0, dword, ptr, [rdi]
 mulss   xmm0, dword, ptr, [rip, +, .LCPI0_0]
 mulss   xmm0, dword, ptr, [rdi, +, 4]
 ret
```

### Non Branching Code:

```bash
$ cargo asm shape::get_area_non_branching
shape::get_area_non_branching:
 movzx   eax, byte, ptr, [rdi, +, 8]
 lea     rcx, [rip, +, .Lswitch.table._ZN5shape22get_area_non_branching17h104da2585c6dbc8eE]
 movss   xmm0, dword, ptr, [rcx, +, 4*rax]
 mulss   xmm0, dword, ptr, [rdi]
 mulss   xmm0, dword, ptr, [rdi, +, 4]
 ret
```
