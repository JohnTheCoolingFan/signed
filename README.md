Things for convenient and human-friendly interaction with signed numbers and their parts

This crate has 0 dependencies aside from std (no_std coming soon?).

```
use signed::Signed;
                                                                                                            
let mut n: i32 = 42; // A number to test operations with it
                                                                                                            
let mut abs_n = n.get_absolute(); // Gets a "pointer" to an absolute value of a number
                                                                                                            
abs_n -= &2;
drop(abs_n);
                                                                                                            
assert_eq!(n, 40);
                                                                                                            
let mut abs_n = n.get_absolute(); // Gets a "pointer" to an absolute value of a number
                                                                                                            
abs_n += &6;
drop(abs_n);
                                                                                                            
assert_eq!(n, 46);
                                                                                                            
n = -42; // Let's try with negative number
                                                                                                            
// Unfortunately, due to assertions, abs_n has to be dropped before immutable use, but when n
// is changed, abs_n is changed accordingly
                                                                                                            
let mut abs_n = n.get_absolute(); // Gets a "pointer" to an absolute value of a number
                                                                                                            
abs_n -= &4;
drop(abs_n);
                                                                                                            
assert_eq!(n, -38); // Now that's different, we subtracted, but got a number *bigger* than the initial one.
                                                                                                            
let mut abs_n = n.get_absolute(); // Gets a "pointer" to an absolute value of a number
                                                                                                            
abs_n += &6;
drop(abs_n);
                                                                                                            
assert_eq!(n, -44); // And by adding to the absolute value, we get a number *smaller*.
                                                                                                            
// All of this works for i8, i16, i32, i64 and i128!
```
