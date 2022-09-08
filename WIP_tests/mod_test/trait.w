disposable(Self) :: trait {
    dispose func(self *Self),
}

rc(T) :: @(no_copy) struct {
    inner *rc_inner(T),
}

rc_inner(T) : struct {
    ref_count usize,
    drop_fn func(*T),

    value T,
}

rc(T):new :: func(val *T, drop_fn func(*T)) rc(T) {
    rc {
        ref_count = 0,
        drop_fn = drop_fn,
        value = val,
    }
}

rc(T):copy :: func(self *rc(T)) option(rc(T)) {
    if self.inner.ref_count == 0 option:none
    else {
        self.inner.ref_count += 1;
        rc {
            inner = self.inner,
        }   
    }
}

rc(T):value :: func(self *rc(T)) option(*T) {
    if self.inner.ref_count == 0 option:none
    else option:some(&self.inner.value)
}

impl(T) :: disposable (rc(T)) {
    dispose func(self *rc (T)) {
        if self.inner.ref_count == 0 return;

        self.inner.ref_count -= 1;
        if self.inner.ref_count == 0
            self.inner.drop_fn(&self.inner.value);
    }
}

test :: func() {
    r := rc:new(1337u32);
    assert(r.value().copied(), option:some(1337));

    r2 := r.copy();
    assert(r.value().copied(), option:some(1337));

    r.dispose();
    assert(r.value().copied(), option:some(1337));

    r2.dispose();
    assert(r.value().copied(), option:none);
}

test2 :: func() {
    r := rc:new(1337u32);
    {
        defer r.dispose();
        assert(r.value().copied(), option:some(1337));
    }
    assert(r.value().copied(), option:none);
}
