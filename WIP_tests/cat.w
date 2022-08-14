{stdin, stdout, read, write} :: std:io;

main :: func()
    mut buf := [1024]u8;

    mut is := stdin();
    mut os := stdout();

    while rl := is.read(&mut buf[..]) && rl != 0 {
        os.write(&buf[..rl]);
    }
}