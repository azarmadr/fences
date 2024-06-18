fn main() {
    let rules = fences::solver::BoardRules::new("assets/rules1.yml").0;
    for (k, v) in rules {
        println!(
            "{}:",
            k.iter_rows()
                .map(|r| r
                    .map(|c| c.map_or(" ".to_string(), |x| x.to_string()))
                    .collect::<Vec<_>>()
                    .join(""))
                .collect::<Vec<_>>()
                .join("\n")
        );
        v.iter().for_each(|r| println!("- {}", r.print()));
    }
}
