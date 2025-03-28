use std::collections::HashMap;

// A structure to store team name and its goal details.
#[derive(Debug)]
pub struct Team {
    pub name: String,
    pub goals_scored: u8,
    pub goals_conceded: u8,
}

pub fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let mut split_iter = r.split(',');
        let team_1_name = split_iter.next().unwrap().trim();
        let team_2_name = split_iter.next().unwrap().trim();
        let team_1_score = split_iter.next().unwrap().trim().parse::<u8>().unwrap();
        let team_2_score = split_iter.next().unwrap().trim().parse::<u8>().unwrap();

        let team_1 = scores.entry(String::from(team_1_name)).or_insert(Team {
            name: String::from(team_1_name),
            goals_scored: 0,
            goals_conceded: 0,
        });
        team_1.goals_scored += team_1_score;
        team_1.goals_conceded += team_2_score;
        let team_2 = scores.entry(String::from(team_2_name)).or_insert(Team {
            name: String::from(team_2_name),
            goals_scored: 0,
            goals_conceded: 0,
        });
        team_2.goals_scored += team_2_score;
        team_2.goals_conceded += team_1_score;
    }
    scores
}
