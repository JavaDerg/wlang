main :: func() {
	a1 := [32]u8;
}

test2 :: option either packet move_event error string;
test2 :: option<either<packet<move_event>, error<string>>;

parse_result P :: result option packet P error string;

parse P (parsable *[]u8 packet P) :: func(data *[]u8) parse_result P {

}

buffer T N :: struct {
	buf [N]T,
}
