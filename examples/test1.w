print_num :: extern "custom_test" func(i32);

main :: func() {
    f := fib(42);
    print_num(f)
}

/* nice
    /*
        embedded comments are good
    */
    :3
*/

fib :: func(n u32) u32 {
    if n <= 2
        n.min(1)
    else
        fib(n - 1) + fib(n - 2)
}
