use regex::Regex;

#[derive(Debug)]
struct BagList {
    amount: usize,
    id: usize,
}

#[derive(Debug)]
struct Bag<'a> {
    name: &'a str,
    id: usize,
    contents: Option<Vec<BagList>>,
}

impl Bag<'_> {
    fn can_contain_gold(&self, rules: &RuleList) -> bool {
        if let Some(contents) = &self.contents {
            for baglist in contents {
                let bag = rules.get_bag(baglist.id);
                if bag.name == "shiny gold" {
                    return true;
                } else if bag.can_contain_gold(&rules) {
                    return true;
                }
            }
        }
        false
    }

    fn bags_inside(&self, rules: &RuleList) -> usize {
        if let Some(contents) = &self.contents {
            contents
                .iter()
                .map(|baglist| {
                    let bag = rules.get_bag(baglist.id);
                    //add for containing bag
                    let int = 1 + bag.bags_inside(&rules);

                    baglist.amount * int
                })
                .sum::<usize>()
        } else {
            0
        }
    }
}

#[derive(Debug)]
struct RuleList<'bag>(usize, Vec<Bag<'bag>>);

impl<'bag> RuleList<'bag> {
    fn new() -> Self {
        Self(0, Default::default())
    }

    fn insert(&mut self, name: &'bag str) {
        self.1.push(Bag {
            id: self.0,
            name,
            contents: None,
        });

        self.0 += 1;
    }

    fn get_bag_by_name_mut(&mut self, name: &'_ str) -> Option<&mut Bag<'bag>> {
        self.1.iter_mut().find(|x| x.name == name)
    }

    fn get_bag_by_name(&self, name: &'_ str) -> Option<&Bag<'bag>> {
        self.1.iter().find(|x| x.name == name)
    }

    fn get_bag(&self, id: usize) -> &Bag<'_> {
        &self.1[id]
    }

    fn can_have_gold(&self) -> usize {
        self.1
            .iter()
            .map(|x| x.can_contain_gold(&self))
            .filter(|x| *x)
            .count()
    }
}

pub fn main() {
    let rules = crate::input(9);

    let mut allbags = RuleList::new();

    let bag_name_regex = Regex::new(r"(.+) bags contain").unwrap();
    let get_bag_name = |rule| {
        bag_name_regex
            .captures(rule)
            .expect("invalid rule")
            .get(1)
            .expect("Missing group")
            .as_str()
    };

    for rule in rules.lines() {
        let name = get_bag_name(&rule);

        allbags.insert(name);
    }

    let bag_count_regex = Regex::new(r"(\d+) (.+?) bag").unwrap();
    for rule in rules.lines() {
        let name = get_bag_name(&rule);

        let mut bags_list = Vec::new();

        for cap in bag_count_regex.captures_iter(rule) {
            let amount: usize = cap[1].parse().unwrap();
            let id = allbags.get_bag_by_name(&cap[2]).unwrap().id;
            bags_list.push(BagList { amount, id });
        }

        let bagref = allbags.get_bag_by_name_mut(&name).unwrap();

        bagref.contents = Some(bags_list);
    }

    dbg!(allbags.can_have_gold());
    let shiny_gold = allbags.get_bag_by_name("shiny gold").unwrap();
    dbg!(shiny_gold.bags_inside(&allbags));
}
