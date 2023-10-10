

Transforms a string as a list of strings separated by `|`` where all combinations of character cases are generated.
By construction first one is fully uppercase, second one is fully lowercase.


The following call:

```rust
match value.as_ref() {
	choice_nocase!("hi") => println!("good"),
	_ => println!("bad")
};
```


corresponds to a similar macro expansion:

```rust
match value.as_ref() {
	"HI" | "hi" | "Hi" | "hI" => println!("good"),
	_ => println!("bad")
};
```

