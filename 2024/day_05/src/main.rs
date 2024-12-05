use std::io::{self, prelude::*};

type OrderingRule = (Page, Page);
type Page = u32;
type UpdateList = Vec<Page>;

fn parse(lines: &[String]) -> (Vec<OrderingRule>, Vec<UpdateList>) {
    let mut ordering_rules = vec![];
    let mut update_lists = vec![];

    for line in lines {
        if line.contains('|') {
            let (l, r) = line.split_once('|').expect("Can't split");
            ordering_rules.push((
                l.parse::<Page>().expect("Not a number"),
                r.parse::<Page>().expect("Not a number"),
            ));
        }

        if line.contains(',') {
            let update_list = line
                .split(',')
                .map(|n| n.parse::<Page>().expect("Not a number"))
                .collect();
            update_lists.push(update_list);
        }
    }

    (ordering_rules, update_lists)
}

fn is_list_in_order(ordering_rules: &[OrderingRule], update_list: &UpdateList) -> bool {
    let mut seen = vec![];

    for item in update_list {
        let pages_following = ordering_rules
            .iter()
            .filter(|(l, _)| l == item)
            .map(|(_, r)| r);

        if pages_following.into_iter().any(|p| seen.contains(p)) {
            return false;
        }

        seen.push(*item);
    }

    true
}

fn get_updates_in_correct_order(
    ordering_rules: &[OrderingRule],
    update_lists: &[UpdateList],
) -> Vec<UpdateList> {
    let result = update_lists
        .iter()
        .filter(|&l| is_list_in_order(ordering_rules, l))
        .cloned()
        .collect();

    // println!("{:?}", result);

    result
}

fn get_middle_page_sum(update_lists: &[UpdateList]) -> u32 {
    update_lists
        .iter()
        .map(|l| {
            let mid_point = l.len() / 2;
            let mid_value = l[mid_point];
            mid_value
        })
        .sum()
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let (ordering_rules, updates) = parse(&lines);

    let ordered_updates = get_updates_in_correct_order(&ordering_rules, &updates);
    let answer = get_middle_page_sum(&ordered_updates);

    println!("Middle page sum of correctly ordered updates is {}", answer);

    Ok(())
}
