use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::io::*;

struct Album<'a> {
    parent: usize,
    name: &'a str,
    child_albums: BTreeMap<&'a str, usize>,
    files: BTreeSet<&'a str>,
}

impl Album<'_> {
    fn remove_album(&mut self, name: &str) -> Option<usize> {
        self.child_albums.remove(name)
    }

    fn get_all_child_album_seq(&self) -> Vec<usize> {
        self.child_albums.keys().filter_map(|name| self.get_child_album_seq(name)).collect()
    }

    fn get_child_album_seq(&self, name: &str) -> Option<usize> {
        self.child_albums.get(name).copied()
    }

    fn get_first_child_album_seq(&self) -> Option<usize> {
        let opt_first_child_name = self.child_albums.keys().next();
        if let Some(first_file_name) = opt_first_child_name {
            self.get_child_album_seq(&first_file_name)
        } else {
            None
        }
    }

    fn get_last_child_album_seq(&self) -> Option<usize> {
        let opt_last_child_name = self.child_albums.keys().next_back();
        if let Some(last_file_name) = opt_last_child_name {
            self.get_child_album_seq(&last_file_name)
        } else {
            None
        }
    }

    fn delete_file(&mut self, name: &str) -> usize {
        self.files.remove(name) as usize
    }

    fn delete_first_file(&mut self) -> usize {
        let opt_first_file_name = self.child_albums.keys().next();
        if let Some(first_file_name) = opt_first_file_name {
            self.delete_file(first_file_name)
        } else {
            0
        }
    }

    fn delete_last_file(&mut self) -> usize {
        let opt_last_file_name = self.child_albums.keys().next_back();
        if let Some(last_file_name) = opt_last_file_name {
            self.delete_file(last_file_name)
        } else {
            0
        }
    }

    fn delete_all_file(&mut self) -> usize {
        let ret = self.files.len();
        self.files.clear();
        ret
    }

    fn get_album_count(&self) -> usize {
        self.child_albums.len()
    }

    fn get_file_count(&self) -> usize {
        self.files.len()
    }
}

impl<'a> Album<'a> {
    fn new(parent: usize, name: &'a str) -> Self {
        Self {
            parent,
            name,
            child_albums: BTreeMap::new(),
            files: BTreeSet::new(),
        }
    }

    fn add_album(&mut self, seq: usize, name: &'a str) -> bool {
        let mut ret = true;
        self.child_albums.entry(name).and_modify(|_| ret = false).or_insert(seq);
        ret
    }

    fn add_file(&mut self, name: &'a str) -> bool {
        self.files.insert(name)
    }
}

fn select_recursive(albums: &mut [Album], seq: usize) -> (usize, usize) {
    let mut album_count = albums[seq].get_album_count();
    let mut file_count = albums[seq].get_file_count();

    for child_seq in albums[seq].get_all_child_album_seq() {
        let rec_result = select_recursive(albums, child_seq);
        album_count += rec_result.0;
        file_count += rec_result.1;
    }

    (album_count, file_count)
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut albums = Vec::with_capacity(n);
    let mut seq = 1;
    let mut cur = 0;

    albums.push(Album::new(0, "album"));

    for _ in 0..n {
        let command = next!();
        let argument = next!();

        match command {
            "mkalb" => {
                if albums[cur].add_album(seq, argument) {
                    albums.push(Album::new(cur, argument));
                    seq += 1;
                } else {
                    let _ = writeln!(output, "duplicated album name");
                }
            }
            "rmalb" => {
                let mut result = (0, 0);
                match argument {
                    "-1" => {
                        if let Some(first_child_seq) = albums[cur].get_first_child_album_seq() {
                            let name = albums[first_child_seq].name;
                            let (album_count, file_count) = select_recursive(&mut albums, first_child_seq);
                            result.0 += album_count + 1;
                            result.1 += file_count;
                            albums[cur].remove_album(name);
                        }
                    }
                    "0" => {
                        for child_seq in albums[cur].get_all_child_album_seq() {
                            let name = albums[child_seq].name;
                            let (album_count, file_count) = select_recursive(&mut albums, child_seq);
                            result.0 += album_count + 1;
                            result.1 += file_count;
                            albums[cur].remove_album(name);
                        }
                    }
                    "1" => {
                        if let Some(last_child_seq) = albums[cur].get_last_child_album_seq() {
                            let name = albums[last_child_seq].name;
                            let (album_count, file_count) = select_recursive(&mut albums, last_child_seq);
                            result.0 += album_count + 1;
                            result.1 += file_count;
                            albums[cur].remove_album(name);
                        }
                    }
                    _ => {
                        if let Some(child_seq) = albums[cur].get_child_album_seq(argument) {
                            let name = albums[child_seq].name;
                            let (album_count, file_count) = select_recursive(&mut albums, child_seq);
                            result.0 += album_count + 1;
                            result.1 += file_count;
                            albums[cur].remove_album(name);
                        }
                    }
                }

                let _ = writeln!(output, "{} {}", result.0, result.1);
            }
            "insert" => {
                if !albums[cur].add_file(argument) {
                    let _ = writeln!(output, "duplicated photo name");
                }
            }
            "delete" => {
                let result = match argument {
                    "-1" => albums[cur].delete_first_file(),
                    "0"  => albums[cur].delete_all_file(),
                    "1"  => albums[cur].delete_last_file(),
                    _    => albums[cur].delete_file(argument),
                };
                let _ = writeln!(output, "{result}");
            }
            "ca" => {
                match argument {
                    ".." => cur = albums[cur].parent,
                    "/" => cur = 0,
                    _ => {
                        if let Some(chile_seq) = albums[cur].get_child_album_seq(argument) {
                            cur = chile_seq;
                        }
                    }
                }
                let _ = writeln!(output, "{}", albums[cur].name);
            }
            _ => unreachable!(),
        }
    }

    print!("{output}");
}