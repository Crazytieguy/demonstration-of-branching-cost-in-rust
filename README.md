# Demonstration of the cost of branching in Rust

This is based on a [video by Molly Rocket](https://www.youtube.com/watch?v=tD5NrevFtbU&t=1214s) where he shows how clean code can lead to poor performance. This is more or less a port of his demonstation regarding 'hiding internal implementation details' to rust.

The branching code is about 3.4x slower than the non-branching code.

## Code

### Branching:

```rust
match shape.shape_type {
    Square => 1.0 * shape.width * shape.height,
    Rectangle => 1.0 * shape.width * shape.height,
    Triangle => 0.5 * shape.width * shape.height,
    Circle => PI * shape.width * shape.height,
}
```

### Non Branching:

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
```

```
ShapeArea/Branching/100000-repetitions
                        time:   [608.63 µs 616.95 µs 625.30 µs]
ShapeArea/NonBranching/100000-repetitions
                        time:   [146.78 µs 149.05 µs 151.31 µs]
```

## Viewing the assembly

### Branching:


```bash
$ cargo asm shape::get_area_branching
```

```assembly
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

### Non Branching:

```bash
$ cargo asm shape::get_area_non_branching
```

```assembly
shape::get_area_non_branching:
 movzx   eax, byte, ptr, [rdi, +, 8]
 lea     rcx, [rip, +, .Lswitch.table._ZN5shape22get_area_non_branching17h104da2585c6dbc8eE]
 movss   xmm0, dword, ptr, [rcx, +, 4*rax]
 mulss   xmm0, dword, ptr, [rdi]
 mulss   xmm0, dword, ptr, [rdi, +, 4]
 ret
```
