// ordering, ord? std stuff idk
sort(T(ord)) :: func(data *[]T) {
	had_update := true
	
	while had_update {
		had_update = false;
		
		for i in 0..(data.len() - 1) {
			if data[i] < data[i + 1] continue;
			
			data.swap(i, i + 1)
			had_update = true;
		}
	}
}
