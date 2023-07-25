fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let binary = input.trim();
    let l = binary.len() / 3 + (binary.len() % 3 != 0) as usize;
    let mut answer = Vec::with_capacity(l);
    let mut binary = binary.bytes().map(|e| e - b'0').rev().enumerate();
    let oc = [1, 2, 4];
    let mut sum = 0;

    loop {
        if let Some((i, elem)) = binary.next() {
            sum += oc[i % 3] * elem;
            if (i + 1) % 3 == 0 {
                answer.push(sum);
                sum = 0;
            }
        } else {
            if sum != 0 || answer.is_empty() {
                answer.push(sum); 
            }
            break;
        }
    }

    println!("{}", String::from_iter(answer.iter().rev().map(|e| (e + b'0') as char)));
}
