# s-parse
A simple recursive descent parser for S-expressions.

#### Usage
Calling `parse::parse()` on a string slice containing 0 or more S-expressions yields a `Vec` containing the parsed expressions, which are represented by the following `parse::SExpr` type:
```rust
pub enum SExpr<'a> {
    SInt(i32),
    SFloat(f32),
    SSym(&'a str),
    SStr(&'a str),
    SList(Vec<SExpr<'a>>)
}
```

The parser can deal with positive/negative integers/floats (floats must have at least one digit before the decimal, however), symbols (which consist of non-whitespace, non-parenthetical consecutive characters), strings, and lists. 

Strings must be within double quotes. Escaped quotes are not supported.

Whitespace is ignored other than as a separator between sub-expressions.

#### Example

```rust
parse::parse("((lambda (x) (* x x)) 50)")
``` 
is parsed as 
```rust
[SList([
  SList([
    SSym("lambda"), 
    SList([
      SSym("x")]), 
    SList([
      SSym("*"), 
      SSym("x"), 
      SSym("x")])]), 
  SInt(50)])]
```
