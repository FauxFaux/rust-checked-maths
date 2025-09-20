# How slow is rust checked maths?

Macbook Pro M3 Max 64GB.

Rust:

```
aaron@MacBook-Pro rust-checked-maths % cargo run --release                                                 
   Compiling rust-checked-maths v0.1.0 (/Users/aaron/codebases/rust-checked-maths)
    Finished `release` profile [optimized] target(s) in 0.36s
     Running `target/release/rust-checked-maths`
rand ints generated in: 6.17595s
sum calculated in: 202.57ms
sum: 4476143
-----------------------------------
i64
rand ints generated in: 9.34796s
sum calculated in: 421.048ms
sum: 500005883205017
aaron@MacBook-Pro rust-checked-maths % 
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