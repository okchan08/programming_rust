use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work)
        }
    }
}

fn sort_table(table: &mut Table) {
    for(_artist, works) in table {
        works.sort();
    }
}

fn main() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(),
                 vec!["many madrings".to_string(), "Tenebrae Responsoria".to_string()]);
    table.insert("Cellini".to_string(),
                 vec!["Persus with the head of Medusa".to_string(), "a salt celler".to_string()]);

    show(&table);
    sort_table(&mut table);
    println!("---- sorted ----");
    show(&table);
}
