**Summary Day-04**

# Struct

- Struct মানে হলো নিজের মতো ডাটা structure বানানো
- যেমন JS-এর object বা Python-এর class-এর data part
- Rust-এ method লিখতে হয় impl ব্লকের ভিতরে।
- &self হলো struct-এর immutable borrow
- JS-এর this বা Python-এর self-এর মতো
- Associated Function (constructor-এর মতো)

# Enum

```rs
enum Direction {
    North,
    South,
    East,
    West,
}
```

# Pattern Matching

- switch এর থেকেও শক্তিশালী।
