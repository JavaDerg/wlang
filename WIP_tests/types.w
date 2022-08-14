    x f32,
    y f32,
    z f32,
}

option(T) :: enum {
    some(T),
    none,
}

opt_vec3 :: option(vec3);

add :: func(a vec3, b vec3) vec3 {
    vec3 {
        x = a.x + b.x,
        y = a.y + b.y,
        z = a.z + b.z,
    }
}

main :: func() {
    // idfk
}
