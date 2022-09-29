{iter, utf8_chk} :: mod;

main :: func() {

}

add_t(A, B, C) :: trait {
	add func(A, B) C,
}

conv_t(A, B) :: trait {
	conv(A) B,
}

add_t(A=add_t(A, A, A), B=conv_t(B, A), A) :: impl {
	add func(a A, b B) C {
		add_t:add(a, conv_t(_, B):conv(b))
	}
}
