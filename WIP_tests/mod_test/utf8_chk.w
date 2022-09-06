{ iter } :: super;

cstr_to_str :: func(cstr *[?]u8) option(*str) {
	len := cstr_len(cstr);
	cstr := &cstr[..len];

	if !utf8_test(cstr) return option:none;

	option:some(cstr as *str) // ??? is str a thing?
}

utf8_test :: func(cstr *[]u8) bool {
	it := iter:new(cstr);
	
	while option:some(char) := it.next() {
		if char & 0x80 == 0 continue;
		
		extra := if char >> 5 == 0b110 1
			else if char >> 4 == 0b1110 2
			else if char >> 3 == 0b11110 3
			else return false;

		for _ in 0..extra
			if option:some(nchar) := it.next()
				if nchar >> 6 == 0b10 continue;
				return false;
			else
				return false;
	}

	true
}

cstr_len :: func(cstr *[?]u8) usize {
	for i in 0.._ if cstr[i] == 0 return i;
	// unreachable
	return 0;
}
