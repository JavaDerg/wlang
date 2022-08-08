// this syntax is experimental and probably wont happen, or maybe it will idfk

compare A B :: trait {
	eq func(a *A, b *B) bool;
	neq func(a *A, b *B) bool {
		!a.eq(b)
	}
}

packet :: struct {
	id u32,
}

impl :: compare packet u32 {
	eq func(a *packet, b *u32) {
		a.id == *b
	}
}

impl A B (compare A B) :: (==) {
	(==) func(left *A, right *B) {
		left.eq(right)
	}
}

main :: func() {
	p1 := packet {
		id = 69,
	};

	p1.eq(&69);
}
