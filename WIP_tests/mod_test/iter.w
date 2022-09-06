iter(T) :: struct {
	data *[]T,
	offset usize,
}

iter(T):new :: func(data *[]T) iter(T) {
	iter {
		data = data,
		offset = 0,
	}
}

iter(T):next :: func(self *iter(T)) option(*T) {
	if self.data.len() >= self.offset option:none
	else {
		tmp := option:some(&self.data[self.offset])
		self.offset += 1;
		tmp
	}
}

iter(T):advance :: func(self *iter(T), by usize) {
	self.offset += by;
}

iter(T):clone :: func(self *iter(T)) iter(T) {
	iter {
		data = self.data,
		offset = self.offset,
	}
}
