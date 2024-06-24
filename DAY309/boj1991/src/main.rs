use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u8);
    let mut tree = [(99, 99); 26];

    for (m, l, r) in (0..n).map(|_| next!(char, char, char)) {
        let m = (m as u8 - b'A') as usize;

        if l != '.' {
            let l = (l as u8 - b'A') as usize;
            tree[m].0 = l;
        }

        if r != '.' {
            let r = (r as u8 - b'A') as usize;
            tree[m].1 = r;
        }
    }

    dfs1(&tree, 0, &mut output);
    output.push('\n');
    dfs2(&tree, 0, &mut output);
    output.push('\n');
    dfs3(&tree, 0, &mut output);

    print!("{output}");
}

fn dfs1(tree: &[(usize, usize)], cur: usize, output: &mut String) {
    output.push((cur as u8 + b'A') as char);

    if tree[cur].0 != 99 {
        dfs1(&tree, tree[cur].0, output);
    }

    if tree[cur].1 != 99 {
        dfs1(&tree, tree[cur].1, output);
    }
}

fn dfs2(tree: &[(usize, usize)], cur: usize, output: &mut String) {
    if tree[cur].0 != 99 {
        dfs2(&tree, tree[cur].0, output);
    }

    output.push((cur as u8 + b'A') as char);

    if tree[cur].1 != 99 {
        dfs2(&tree, tree[cur].1, output);
    }
}

fn dfs3(tree: &[(usize, usize)], cur: usize, output: &mut String) {
    if tree[cur].0 != 99 {
        dfs3(&tree, tree[cur].0, output);
    }

    if tree[cur].1 != 99 {
        dfs3(&tree, tree[cur].1, output);
    }

    output.push((cur as u8 + b'A') as char);
}