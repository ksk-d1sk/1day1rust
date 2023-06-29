fn main() {
    {
        let parabola = [0, 1, 1, 2, 3, 5, 8, 13];
        let s = smallest(&parabola);
        println!("{}", s);
    }

    let s;
    {
        let x = 10;
        s = S{ r: &x };
        println!("{:?}", s);
    }
}

fn smallest<'a>(v: &'a [i32]) -> &'a i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}

#[derive(Debug)]
struct S<'a> {
    r: &'a i32
}

struct D<'a> {
    s: S<'a>
}
