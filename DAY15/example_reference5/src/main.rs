fn main() {
    let v = vec![4, 8, 19, 34, 10];
    let r = &v;
    let aside = v;
    // r[0];     // this code is error!

    let v = vec![4, 8, 19, 34, 10];
    {
        let r = &v;
        r[0];
    }
    let aside = v;

/////////////////////////////////////////////////

    let mut wave = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = [0.0, -1.0];

    extend(&mut wave, &head);

    extend(&mut wave, &tail);

    assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0]);

/////////////////////////////////////////////////

    let mut w = (107, 109);
    let r = &w;
    let r0 = &r.0;
    // let m1 = &mut r.1;      // t his code is error!
    println!("{}", r0);
}

fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for elt in slice {
        vec.push(*elt);
    }
}
