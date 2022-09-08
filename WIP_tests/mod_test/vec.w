vec(T) : struct {
	buf *[]T,
	used usize,
}

vec(T):new :: func() vec(T) {
	vec {
		// we can have as many zero size types as we want lol
		buf = if T::size_of() == 0 alloc_slice(!0)
			else alloc_slice(1024), // 1024 because idk
		used = 0,
	}
}

vec(T):push :: func(self *vec(T), val T) {
	if self.buf.len() >= self.used {
		// realloc maybe idk?
		new := alloc_slice(self.buf.len() * 2);
	
		new.copy_from(self.buf);

		free(self.buf);
		// who needs stable references anyways lmao
		self.buf = new;
	}

	self.buf[self.used] = val;
	self.used += 1;
}

vec(T):pop :: func(self *vec(T)) option(T) {
	if self.used == 0 option:none
	else {
		self.used -= 1;
		option:some(self.buf[self.used])
	}
}

/// modifying the vector may invalidate previous references
/// don't keep references alive, they will change
vec(T):get :: func(self *vec(T), idx usize) option(*T) {
	if self.used > idx option:some(&self.buf[idx])
	else option:none
}

// experimental idk
vec(T(disposable)):dispose :: func(self *vec(T)) {
	for i in 0..used
		self.buf[i].dispose();

	free(self.buf);
	self.buf = 0;
}
