# Usage
Add this to your Cargo.toml:

```
[dependencies]
vec_rand = "0.1.0"
```

```
let len = 10;

// Pass the type you want to generate to Generics.
// RandVec::generate<i32>(len);
let v = RandVec::generate<u8>(len);

println!(":?", v);
// [107, 58, 138, 135, 112, 216, 235, 226, 68, 104]
```
