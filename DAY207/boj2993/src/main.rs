fn main() {
    let mut input  = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let mut answer = "zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz".to_string();

    for i in 1..(input.len() - 1) {
        for j in (i + 1)..input.len() {
            let mut word = input.to_string();
            let arr = unsafe { word.as_bytes_mut() };
        
            reverse(arr.get_mut(..i).unwrap());
            reverse(arr.get_mut(i..j).unwrap());
            reverse(arr.get_mut(j..).unwrap());
        
            if word < answer {
                answer = word;
            }
        }
    }

    print!("{answer}");
}

fn reverse(arr: &mut [u8]) {
    if arr.len() > 1 {
        (arr[0], arr[arr.len() - 1]) = (arr[arr.len() - 1], arr[0]);
        let len = arr.len() - 1;
        reverse(&mut arr[1..len]);
    }
}