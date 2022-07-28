// array ptrs are allways wide pointers, they carry size information
memset_classic_for :: func(ptr *[]u8, val u8) {
	// iterators?
	for i := 0; i < ptr.len; i += 1 {
		ptr[i] = val;
	}
}
