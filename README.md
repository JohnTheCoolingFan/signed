Things for convenient and human-friendly interaction with signed numbers and their parts

```rs
use signed::Signed;

let mut n: i32 = 42; // A number to test operations with it

let mut abs_n = n.get_absolute(); // Gets a "pointer" to an absolute value of a number

abs_n -= 2;

assert_eq!(n, 40);

abs_n += 6;

assert_eq!(n, 46);

n = -42; // Let's try with negative number

// abs_n still points to the same number, so we don't have to create a new one!

abs_n -= 4;

assert_eq!(n, -38); // Now that's different, we subtracted, but got a number *bigger* than the initial one.

abs_n += 6;

assert_eq!(n, -44); // And by adding to the absolute value, we get a number *smaller*.

// All of this works for i8, i16, i32, i64 and i128!
```
