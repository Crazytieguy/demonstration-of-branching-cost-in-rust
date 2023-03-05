# Demonstration of the optimizability of match statements in rust

This is based on a [video by Molly Rocket](https://www.youtube.com/watch?v=tD5NrevFtbU&t=1214s) where he shows how clean code can lead to poor performance. This is more or less a port of his demonstation regarding 'hiding internal implementation details' to rust.

## Results

The branching code takes about 5.8 ns per iteration, while the non-branching code takes about 1.7 ns per iteration. So around 3.4x faster.

The branching code:

```rust
match shape.shape_type {
    Square => 1.0 * shape.width * shape.height,
    Rectangle => 1.0 * shape.width * shape.height,
    Triangle => 0.5 * shape.width * shape.height,
    Circle => PI * shape.width * shape.height,
}
```

Non branching code:

```rust
let coefficient = match shape.shape_type {
    Square => 1.0,
    Rectangle => 1.0,
    Triangle => 0.5,
    Circle => PI,
};
coefficient * shape.width * shape.height
```
