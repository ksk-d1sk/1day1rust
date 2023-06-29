fn main() {
    let x = 10;
    let a;
    let b;
    {
        let y = 20;
        {
            let s = S {
                x: &x,
                y: &y,
            };
            a = s.x;
            b = s.y;
        }
        println!("{}", b);
    }
    println!("{}", a);
}

struct S<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

fn f<'a, 'b>(r: &'a i32, s: &'b i32) -> &'a i32 {
    r
}

fn sum_r_xy(r: &i32, s: S) -> i32 {   /* is same */   // fn sum_r_xy<'a, 'b, 'c>(r: &'ai32, s: S<'b, 'c>) -> i32 {
    r + s.x + s.y
}

fn first_third(point: &[i32; 3]) -> (&i32, &i32) {   /* is same */   // fn first_third<'a>(point: &'a [i32; 3]) -> (&'a i32, &'a i32) {
    (&point[0], &point[2])
}

struct StringTable {
    elements: Vec<String>,
}

impl StringTable {
    fn find_by_prefix(&self, prefix: &str) -> Option<&String> {   /* is same */   // fn find_by_prefix<'a, 'b>(&'a self, prefix: &'b str) -> Option<&'a String> {
        let mut answer = None;
        for i in 0..self.elements.len() {
            if self.elements[i].starts_with(prefix) {
                answer = Some(&self.elements[i]);
            }
        }
        answer
    }
}