// this syntax is experimental and probably wont happen, or maybe it will idfk

printable Self :: trait {
	as_string func(self *Self) string;
	println func(self *Self) {
		println(self.as_string);
	}
}

test :: struct {
	num u32,
}

impl :: printable test {
	as_string func(self *test) string {
		num.as_string()
	} 
}
