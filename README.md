## A pipe operator-like experience in Rust.

This crate provides a handy [`Pipe`][trait.Pipe] trait that you can implement for arbitrary types
to use them as pipes in a pipeline.

### Usage

In the example below the [`pipe`][fn.pipe] function generates a [`PartialPipeline`][struct.PartialPipeline]
which can be used to "complete" (invoke) *any* type that implements [`Pipe`][trait.Pipe] using the
shift right (`>>`) operator.

```rust
use tuppipe::pipe;

const fn add_one(to: i32) -> i32 {
    to + 1
}

assert_eq!(2, pipe(0) >> (add_one, add_one));
```

The first element in the tuple (the tuple being the pipeline) after `>>`, will receive the `0i32`
that you see in the `pipe` invocation. The second element in the pipeline will receive the output
from the first element in the pipeline, and so on.

### Default implementations of [`Pipe`][trait.Pipe]

Note that [`Pipe`][trait.Pipe] is currently implemented for both, anything that implements `FnOnce` 
from the standard library *and* for any tuple with up to 16 elements where each element in the tuple
itself implements [`Pipe`][trait.Pipe] itself as well.

This means that if you really want a pipeline that's longer than a tuple of 16 elements, you can
pretty much infinitely nest tuples in one another.

---

You can check out the [docs](https://docs.rs/tuppipe) for more documentation and examples.

[struct.PartialPipeline]: https://docs.rs/tuppipe/latest/tuppipe/struct.PartialPipe.html
[trait.Pipe]: https://docs.rs/tuppipe/latest/tuppipe/trait.Pipe.html
[fn.pipe]: https://docs.rs/tuppipe/latest/tuppipe/fn.pipe.html
