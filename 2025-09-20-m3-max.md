# How slow is rust checked maths?

Macbook Pro M3 Max 64GB.

Rust:

```
aaron@MacBook-Pro rust-checked-maths % cargo run --release
   Compiling rust-checked-maths v0.1.0 (/Users/aaron/codebases/rust-checked-maths)
    Finished `release` profile [optimized] target(s) in 0.38s
     Running `target/release/rust-checked-maths`
-----------------------------------
Unchecked maths performance test
-----------------------------------
i32
rand ints generated in: 6.461324s
sum calculated in: 557.042ms
sum: 1304940430
-----------------------------------
i64
rand ints generated in: 9.256838s
sum calculated in: 1.154704s
sum: 499994696406534


-----------------------------------
CHECKED maths performance test
-----------------------------------
i32
overflow occurred
sum calculated in: 1.16375s
sum: Some(499999922704270)
-----------------------------------
i64
sum calculated in: 886.603ms
sum: Some(499994696406534)
```


ngn-k:

```
 \t a:1000000000?1000000
3353
 \t s:+/a
128
 s
500003394377056
 \t a:1000000000?8000000000
4198
 \t s:+/a
122
 s
3999967211669783079
 
```