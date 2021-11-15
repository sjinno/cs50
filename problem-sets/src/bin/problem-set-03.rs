extern crate prettytable;
use prettytable::Table;

use failure::Error;
use problem_sets::input;

fn main() -> Result<(), Error> {
    // Load list of candidates
    let file = "src/bin/problem-set-03.input";
    let data = input::load_data(&file)?;
    let candidates: Vec<String> = data.map(|c| c.unwrap().to_ascii_lowercase()).collect();
    eprintln!("candidates {:#?}", candidates);

    // Initialize tideman
    let mut tideman = Tideman::new(candidates);
    // eprintln!("number_of_voters {}", tideman.number_of_voters);

    // eprintln!("TIDEMAN {:#?}", tideman);

    // Take votes from voters
    for _vote in 0..tideman.number_of_voters {
        let mut ranks = Vec::<String>::with_capacity(tideman.number_of_voters); // Helps keep track of cadidates that the voter has already voted
        for rank in 0..tideman.candidates.len() {
            loop {
                let name = input::get_input(&format!("Rank {}: ", rank + 1))
                    .trim()
                    .to_ascii_lowercase();
                eprintln!("name {}", name);

                if tideman.vote(rank, name, &mut ranks) {
                    break;
                }
            }
        }
        // println!("{:?}", ranks);
    }

    // eprintln!("prefs {:#?}", tideman.preferences);

    tideman.show_preferences_table();
    tideman.print_winner();

    Ok(())
}

#[allow(dead_code)]
fn capitalize(name: &str) -> String {
    format!("{}{}", &name[..1].to_ascii_uppercase(), &name[1..])
}

#[derive(Debug, Default)]
struct Pair {
    winner: usize, // Candidate's index, not the number of votes they received
    loser: usize,
}

#[derive(Debug, Default)]
struct Tideman {
    candidates: Vec<String>,
    number_of_voters: usize,
    preferences: Vec<Vec<u8>>,
    locked: Vec<Vec<bool>>,
    pairs: Vec<Pair>,
    pair_count: usize,
}

#[allow(dead_code, unused_variables)]
impl Tideman {
    fn new(candidates: Vec<String>) -> Self {
        let number_of_voters = input::get_number_input("Number of voters: ");
        let preferences = vec![vec![0; candidates.len()]; candidates.len()];
        let locked = vec![vec![false; candidates.len()]; candidates.len()];

        Self {
            candidates,
            number_of_voters,
            preferences,
            locked,
            pairs: Default::default(),
            pair_count: 0,
        }
    }

    fn vote(&mut self, rank: usize, name: String, ranks: &mut Vec<String>) -> bool {
        // Check if the name exists in the list
        if self.candidates.contains(&name) {
            // Check if the candidate was already voted
            if !ranks.contains(&name) {
                // Update already voted list
                ranks.push(name);
                // Update `preferences`
                self.record_preferences(&ranks);
                true
            } else {
                println!("The candidate has already been voted. Try again.");
                false
            }
        } else {
            println!("The name does not exist in the list. Try again.");
            false
        }
    }

    fn record_preferences(&mut self, ranks: &Vec<String>) {
        if ranks.len() == self.preferences[0].len() {
            return;
        }

        let name = &ranks[ranks.len() - 1];
        let candidate_index = self.candidates.iter().position(|n| n == name).unwrap();
        // eprintln!("index of {} = {:?}", ranks[ranks.len() - 1], index);

        for (idx, vote) in self.preferences[candidate_index].iter_mut().enumerate() {
            if ranks.contains(&self.candidates[idx]) {
                continue;
            }
            *vote += 1;
        }
    }

    fn add_pairs(&mut self) {
        for i in 0..self.candidates.len() / 2 + 1 {
            for j in i + 1..self.candidates.len() {
                if self.preferences[i][j] > self.preferences[j][i] {
                    self.pairs.push(Pair {
                        winner: i,
                        loser: j,
                    });
                    self.pair_count += 1;
                } else if self.preferences[i][j] < self.preferences[j][i] {
                    self.pairs.push(Pair {
                        winner: j,
                        loser: i,
                    });
                    self.pair_count += 1;
                }
            }
        }
        // eprintln!("PAIRS {:?}", self.pairs);
    }

    fn sort_pairs(&mut self) {
        let pref = &self.preferences;

        self.pairs.sort_by(|a, b| {
            pref[b.winner][b.loser]
                .partial_cmp(&pref[a.winner][a.loser])
                .unwrap()
        });
        // eprintln!("SORTED PAIRS {:?}", self.pairs);
    }

    fn lock_pairs(&mut self) -> usize {
        let mut losers = Vec::<usize>::new();

        let mut winner = self.pairs[0].winner;
        let loser = self.pairs[0].loser;
        losers.push(loser);

        self.locked[winner][loser] = true;
        self.locked[loser][winner] = false;

        for pair in self.pairs.iter().skip(1) {
            let w = pair.winner;
            let l = pair.loser;

            if !losers.contains(&pair.winner) && pair.loser == winner {
                losers.push(winner);
                winner = pair.winner;
                self.locked[w][l] = true;
                self.locked[l][w] = false;
            }
        }

        winner
    }

    fn print_winner(&mut self) {
        self.add_pairs();
        self.sort_pairs();
        let winner_index = self.lock_pairs();
        println!("{}", capitalize(&self.candidates[winner_index]));
    }

    fn show_preferences_table(&self) {
        let mut table = Table::new();
        for row in self.preferences.iter() {
            table.add_row(row.into());
        }
        table.printstd();
    }
}
