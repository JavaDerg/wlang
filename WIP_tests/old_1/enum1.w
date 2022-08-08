result T E :: enum {
    ok T,
    err E,
}

test :: func() result u32 string {
    ok 69
}

main :: func() {
    res := test();

    // imagine having switch/match lul
    // println nor strings nor adding strings together or number etc is official
    println(
        if ok &69 := &res
            "funny number"
        else if ok &420 := &res
            "weed"
        else if ok t := &res
            "got num: " + t
        else if err e := &res
            "err: " + e
    );
}