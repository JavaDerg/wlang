option(T) :: enum {
    some(t),
    none,
}

some_test :: option(str) := test("test");

// const?
test :: func(s str) option(str) {
    option::some(s)
}

main :: func() {
    t := some_test;
}
