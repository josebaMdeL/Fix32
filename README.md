# Fix32 V1.1

Easy implementation of fixed point numbers in Rust, using i32 as base. Decimal places should be configured at compile time, to be able to optimize the code at max. This is a personal project and is meant to keep it clear and easily understandable. Anyone can commit improvements or extra characrterisitcs, but all of them have to be clearly explained for a non profesional programmer. I will not add any code I don't understand.

## Considerations on internal maths

All the operations that convert from float types to int will truncate value instead of roinding.

## Considerations on version numbering

Mayor number in verion will lock all the features that it has and works. Minor version number will imply improvements on the code or bug corrections, but no new features.

# Features

## V1

* i32 based fixed point struct
* Creation from basic tipes (i32, i64, f32, f64), new with 0 value and direcyly setting internal i32 value.
* Get value in basic types and String.
* Basic operator overloading: (+,-,*,/,% and comparing operators)
* Basic math functions:
  * Get remainder as i32.
  * Truncate internal decimals (changing self value)
  * Get absolute value.
  * Get powered to 2, 3 or any other positive integer.

## V2

* Sin, cos and tan funtcions based on lookup tables.
* Many more useful numeric constants in separate file/mod.
* **sqrt** function (find algorithm that works only with integer values).
* Start adding 2D geometry set of functions (new module).
* Add more testing cases.
* Make basic documentation

## TODO

### Next version features

* Sin, cos and tan funtcions based on lookup tables.
* Many more useful numeric constants in separate file/mod.
* **sqrt** function (find algorithm that works only with integer values).
* Start adding 2D geometry set of functions (new module).
* Add more testing cases.
* Make basic documentation

### General

* Find a way to adjust decimal positions on compile time. ¿Will ever be possible on Rust?
* Code comments for documentation using Cargo Doc.
* Many more tests for everything
* Think of some benchmarking system to detect performance problems.

### Aid functions

* (Nothing)

## Bugs, dangers and limitations

* Rounding errors on making power functions or multiple multiplciations if operands are small number with many decimals. Maybe are not avoidable without heavy performance drop.
* Spellcheck ;)

# License

This software us licensed under License GPL-3.0-only.

Copyright Joseba Martinez 2020  (josebam@protonmail.com).

