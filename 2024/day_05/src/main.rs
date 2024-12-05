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
        let page_rules_following = ordering_rules
            .iter()
            .filter(|(l, _)| l == item)
            .map(|(_, r)| r);

        if page_rules_following.into_iter().any(|p| seen.contains(p)) {
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

fn order_update(ordering_rules: &[OrderingRule], update: &UpdateList) -> UpdateList {
    let mut ordered_list: UpdateList = update.clone();

    let mut i = 1;

    while i < update.len() {
        let numbers_to_left = ordered_list[0..i].to_vec();

        let item = ordered_list[i];

        let page_rules_following = ordering_rules
            .iter()
            .filter(|(l, _)| *l == item)
            .map(|(_, r)| r)
            .collect::<Vec<_>>();

        if let Some(index) = numbers_to_left
            .iter()
            .position(|p| page_rules_following.contains(&p))
        {
            // println!("---");
            // println!("Numbers to left {:?}", numbers_to_left);
            // println!("Before {:?}", ordered_list);

            // println!(
            //     "Item {} is in the wrong place, because of {}",
            //     item, ordered_list[index],
            // );

            ordered_list.swap(index, i);

            // println!("After {:?}", ordered_list);
            // println!("---");
            i = index
        } else {
            i += 1
        }
    }

    ordered_list
}

fn order_unordered_updates(
    ordering_rules: &[OrderingRule],
    update_lists: &[UpdateList],
) -> Vec<UpdateList> {
    let unordered_updates = update_lists
        .iter()
        .filter(|&l| !is_list_in_order(ordering_rules, l));

    let ordered_updates = unordered_updates
        .map(|u| order_update(ordering_rules, u))
        .collect();

    // println!("{:?}", ordered_updates);

    ordered_updates
}

fn get_middle_page_sum(update_lists: &[UpdateList]) -> u32 {
    update_lists
        .iter()
        .map(|l| {
            let mid_point = l.len() / 2;
            l[mid_point]
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

    let ordered_unordered_updates = order_unordered_updates(&ordering_rules, &updates);
    let answer2 = get_middle_page_sum(&ordered_unordered_updates);

    println!(
        "Middle page sum of incorrectly ordered then order updates is {}",
        answer2
    );

    Ok(())
}
