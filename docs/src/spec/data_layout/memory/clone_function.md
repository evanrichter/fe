# The `clone` function

Reference type values in memory can be cloned using the `clone` function.

Example:

```fe
fn f() {
    // with clone
    let bar: Array<u256, 4> = [1, 2, 3, 4]
    let foo: Array<u256, 4> = bar.clone() // `foo` points to a new segment of memory
    assert foo[1] == bar[1]
    foo[1] = 42
    assert foo[1] != bar[1] // modifying `foo` does not modify bar
}

fn g() {
    // without clone
    let bar: Array<u256, 4> = [1, 2, 3, 4]
    let foo: Array<u256, 4> = bar // `foo` and `bar` point to the same segment of memory
    assert foo[1] == bar[1]
    foo[1] = 42
    assert foo[1] == bar[1] // modifying `foo` also modifies `bar`
}
```
