use std::collections::HashMap;

pub fn main() {
    // HashMap<K, V>
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // its important to note that hash maps take ownership of values placed in them
    let blue_scores = scores.get("Blue");
    println!("this is the blue teams score: {:#?}", blue_scores); // keep in mind this will return a Some() Type

    for (key, value) in &scores {
        println!("{} : {}", key, value);
    }

    // if we insert a value on a key that already has a value, the previously held value will be overwritten
    scores.insert("Blue".to_string(), 25); // blue score is now 25 instead of 10
                                           // to check if a key has a value assigned we can use .entry
    scores.entry(String::from("Blue")).or_insert(45); // this will do nothing as blue already has a value

    let text: String = String::from("HellO world world hey world");

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; // we either insert 0 or receive 0 and add 1 to it
    }

    // rustlings hashmap3.rs
    #[derive(Debug)]
    struct Team {
        name: String,
        goals_scored: u8,
        goals_conceded: u8,
    }

    fn build_scores_table(results: String) -> HashMap<String, Team> {
        // The name of the team is the key and its associated struct is the value.
        let mut scores: HashMap<String, Team> = HashMap::new();

        for r in results.lines() {
            let v: Vec<&str> = r.split(',').collect();
            let team_1_name = v[0];
            let team_1_score: u8 = v[2].parse().unwrap();
            let team_2_name = v[1];
            let team_2_score: u8 = v[3].parse().unwrap();

            let team1 = Team {
                name: team_1_name.to_string(),
                goals_scored: team_1_score,
                goals_conceded: team_2_score,
            };

            scores
                .entry(team_1_name.to_string())
                .and_modify(|team1| {
                    team1.goals_scored += team_1_score;
                    team1.goals_conceded += team_2_score;
                })
                .or_insert(team1);

            let team2 = Team {
                name: team_2_name.to_string(),
                goals_scored: team_2_score,
                goals_conceded: team_1_score,
            };

            scores
                .entry(team_2_name.to_string())
                .and_modify(|team2| {
                    team2.goals_scored += team_2_score;
                    team2.goals_conceded += team_1_score
                })
                .or_insert(team2);

            // TODO: Populate the scores table with details extracted from the
            // current line. Keep in mind that goals scored by team_1
            // will be number of goals conceded from team_2, and similarly
            // goals scored by team_2 will be the number of goals conceded by
            // team_1.
        }
        scores
    }

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    // spain 0 2
    // Englad 5 4
    // France 5 5
    // germany 2 1
    // italy 1 3
    // poland 2 0

    let scores = build_scores_table(get_results());
    println!("Scores: {:#?}", scores);
}
