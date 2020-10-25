# Lisp in Rust by using traits

Like literally within Rust. Not a Lisp interpreter. We're using Rust tuples
here.

Thank [this blog post](https://samwho.dev/blog/fun-with-rust-traits/) by
@samwhoo for this tyranny.

Also I modified it to use an inline proc macro to generate the lisp function Node `impl`'s
for me. Because that lets me have like up to any amount of arguments for a function,
in addition to atrocious compile times.
