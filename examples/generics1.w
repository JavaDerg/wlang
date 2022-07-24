extern "__buildin__" trap :: func() !;
extern "custom_test" printnum :: func(u32);

main :: func() {
    a := some(69)
    b := a.bind(func(a) {
            t := a * 69
            if t > 5234324
                none
            else
                some(t)
        }).map(func(a) {
            a - 420
        });

    printnum(b.unwrap());
}

option A :: enum {
    some(A),
    none,
}

map A B :: func(A) B;

map A B :: func(self option A, f map A B) option B {
    if some(a) := self
        some(f(a))
    else
        none
}

bind A B :: func(self option A, f map A option B) option B {
    if some(a) := self
        f(a)
    else
        none
}

unwrap A :: func(self option A) A {
    if some(a) := self
        a
    else
        trap()
}
