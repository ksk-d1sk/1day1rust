struct Class {
    class_name: &'static str,
    population: i64,
    rank: i32,
}

impl Class {
    fn print(&self) {
        println!("{}, 인구수: {}", self.class_name, self.population);
    }
}

fn count_selected_class<F>(classes: &Vec<Class>, test_fn: F) -> usize
where
    F: Fn(&Class) -> bool,
{
    let mut count = 0;
    for class in classes {
        if test_fn(class) {
            count += 1;
        }
    }
    count
}

fn has_3rank_more(class: &Class) -> bool {
    class.rank > 2
}

fn class_population_descending(class: &Class) -> i64 {
    -class.population
}

fn sort_classes(classes: &mut [Class]) {
    // 동일한 기능을 수행하는 코드
    classes.sort_by_key(class_population_descending);
    classes.sort_by_key(|class| -class.population);
}

fn main() {
    let mut my_classes = vec![
        Class { class_name: "햇님반"  , population: 5_200_000_000, rank: 5 },
        Class { class_name: "달님반"  , population:    47_000_000, rank: 4 },
        Class { class_name: "무궁화반", population: 2_100_000_000, rank: 3 },
        Class { class_name: "개나리반", population: 7_701_000_000, rank: 2 },
        Class { class_name: "반다크홈", population:       666_666, rank: 1 },
    ];
    sort_classes(&mut my_classes);
    for class in my_classes.iter() {
        class.print();
    }

    let n = count_selected_class(     // Ok
        &my_classes,
        has_3rank_more
    );
    assert_eq!(n, 3);

    let n = count_selected_class(     // Ok
        &my_classes,
        |class| class.rank > 3,
    );
    assert_eq!(n, 2);
}
