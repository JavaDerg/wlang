entity :: struct {
	id string,
	age u32,
}

oldest :: func(a entity, b entity) entity {
	if a.age > b.age
		a
	else
		b
}

main :: func() {
	java := entity {
		id = 69,
		age = 20,
	};
	lambda := entity {
		id = 420,
		age = 50,
	};

	oldest := java.oldest(lambda);
	// print?
}
