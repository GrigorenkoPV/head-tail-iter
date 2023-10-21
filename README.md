# Head-Tail-Iter

An iterator that splits a slice into its head and tail.
Then, the tail into *its* head and tail, and so on.

Iterator's element type is `(&'a T, &'a [T])`.
Iteration continues until there are no elements left.
The last yielded value, thus,
has the last element of the original slice as its head,
and an empty slice as its tail.

# Examples

```rust
use std::fmt::Write;
use head_tail_iter::HeadTailIterator;

let mut s = String::new();
for x in [0, 1, 2, 3].head_tail_pairs() {
    writeln!(&mut s, "{:?}", x);
}
assert_eq!(s, "\
(0, [1, 2, 3])
(1, [2, 3])
(2, [3])
(3, [])
");
```

```rust
use head_tail_iter::HeadTailIterator;

for (head, tail) in vec![3, 2, 1, 0].head_tail_pairs() {
    assert_eq!(*head, tail.len());
}
```
