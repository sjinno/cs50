use failure::Error;

fn main() -> Result<(), Error> {
    let mut tideman = tideman::TidemanBuilder::new()
        .load_candidates("src/bin/problem-set-03.input")?
        .build();

    tideman.begin();

    Ok(())
}

mod tideman {
    use failure::Error;
    use prettytable::Table;
    use problem_sets::input;
    use std::cmp::Ordering;
    use std::fmt::{self, Display};

    #[allow(dead_code)]
    fn capitalize(name: &str) -> String {
        // Assuming that `name` has at least 2 letters
        format!("{}{}", &name[..1].to_ascii_uppercase(), &name[1..])
    }

    #[derive(Debug, Clone)]
    enum Lock {
        Win,
        Lose,
        Draw,
    }

    impl Display for Lock {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Lock::Win => f.write_str("true"),
                Lock::Lose => f.write_str("false"),
                Lock::Draw => f.write_str("draw"),
            }
        }
    }

    #[derive(Debug, Default, PartialEq)]
    struct Pair {
        winner: usize, // Candidate's index, not the number of votes they received
        loser: usize,
    }

    #[derive(Debug, Default)]
    pub struct Tideman {
        candidates: Vec<String>,
        number_of_voters: usize,
        preferences: Vec<Vec<u8>>,
        locked: Vec<Vec<Lock>>,
        pairs: Vec<Pair>,
        pair_count: usize,
    }

    impl Tideman {
        pub fn begin(&mut self) {
            // Take votes from voters
            for _vote in 0..self.number_of_voters {
                let mut ranks = Vec::<String>::with_capacity(self.number_of_voters); // Helps keep track of cadidates that the voter has already voted
                for rank in 0..self.candidates.len() {
                    loop {
                        let name = input::get_input(&format!("Rank {}: ", rank + 1))
                            .trim()
                            .to_ascii_lowercase();

                        if self.vote(name, &mut ranks) {
                            break;
                        }
                    }
                }
                println!();
            }

            self.show_preferences_table();
            self.print_winner();
        }

        fn vote(&mut self, name: String, ranks: &mut Vec<String>) -> bool {
            // Check if the name exists in the list
            if self.candidates.contains(&name) {
                // Check if the candidate was already voted
                if !ranks.contains(&name) {
                    // Update already voted list
                    ranks.push(name);
                    // Update `preferences`
                    self.record_preferences(ranks);
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

        fn record_preferences(&mut self, ranks: &[String]) {
            if ranks.len() == self.preferences[0].len() {
                return;
            }

            let name = &ranks[ranks.len() - 1];
            let candidate_index = self.candidates.iter().position(|n| n == name).unwrap();

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
                    match self.preferences[i][j].cmp(&self.preferences[j][i]) {
                        Ordering::Less => {
                            self.pairs.push(Pair {
                                winner: j,
                                loser: i,
                            });
                            self.pair_count += 1;
                        }
                        Ordering::Greater => {
                            self.pairs.push(Pair {
                                winner: i,
                                loser: j,
                            });
                            self.pair_count += 1;
                        }
                        Ordering::Equal => {}
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

        fn lock_pairs(&mut self) -> Option<usize> {
            if self.pairs.is_empty() {
                return None;
            }

            (0..self.candidates.len()).for_each(|i| self.locked[i][i] = Lock::Lose);

            let mut losers = Vec::<usize>::new();

            let mut winner = self.pairs[0].winner;
            let loser = self.pairs[0].loser;
            losers.push(loser);

            self.locked[winner][loser] = Lock::Win;
            self.locked[loser][winner] = Lock::Lose;

            for pair in self.pairs.iter().skip(1) {
                let w = pair.winner;
                let l = pair.loser;

                if pair == &self.pairs[self.pairs.len() - 1] && pair.loser == winner {
                    if !losers.contains(&pair.winner) {
                        winner = pair.winner;
                        self.locked[w][l] = Lock::Win;
                        self.locked[l][w] = Lock::Lose;
                    } else {
                        self.locked[l][w] = Lock::Win;
                        self.locked[w][l] = Lock::Lose;
                    }
                    break;
                }

                self.locked[w][l] = Lock::Win;
                self.locked[l][w] = Lock::Lose;

                if !losers.contains(&pair.winner) && pair.loser == winner {
                    losers.push(winner);
                    winner = pair.winner;
                }
            }

            for i in 0..self.candidates.len() {
                for j in 0..self.candidates.len() {
                    if matches!(self.locked[i][j], Lock::Draw) {
                        if winner == i {
                            self.locked[i][j] = Lock::Win;
                            self.locked[j][i] = Lock::Lose;
                        } else if winner == j {
                            self.locked[j][i] = Lock::Win;
                            self.locked[i][j] = Lock::Lose;
                        } else {
                            self.locked[i][j] = Lock::Lose;
                            self.locked[j][i] = Lock::Lose;
                        }
                    }
                }
            }

            Some(winner)
        }

        fn print_winner(&mut self) {
            self.add_pairs();
            self.sort_pairs();
            if let Some(winner_index) = self.lock_pairs() {
                self.show_locked_table();
                println!("{}", capitalize(&self.candidates[winner_index]));
            } else {
                println!("NO WINNER!");
            }
        }

        fn show_preferences_table(&self) {
            let mut table = Table::new();
            for row in self.preferences.iter() {
                table.add_row(row.into());
            }
            table.printstd();
            println!();
        }

        fn show_locked_table(&self) {
            let mut table = Table::new();
            for row in self.locked.iter() {
                table.add_row(row.into());
            }
            table.printstd();
            println!();
        }
    }

    #[derive(Debug, Default)]
    pub struct TidemanBuilder {
        candidates: Vec<String>,
        number_of_voters: usize,
    }

    impl TidemanBuilder {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn load_candidates(mut self, file: &str) -> Result<Self, Error> {
            let data = input::load_data(file)?;
            self.candidates = data.map(|c| c.unwrap().to_ascii_lowercase()).collect();
            println!(
                "Candidates: {}",
                self.candidates
                    .iter()
                    .map(|c| format!("{}, ", capitalize(c)))
                    .collect::<String>()
            );
            println!();
            Ok(self.get_number_of_voters())
        }

        fn get_number_of_voters(self) -> Self {
            Self {
                number_of_voters: input::get_number_input("Number of voters: "),
                ..self
            }
        }

        pub fn build(self) -> Tideman {
            self.into()
        }
    }

    impl From<TidemanBuilder> for Tideman {
        fn from(builder: TidemanBuilder) -> Self {
            let preferences = vec![vec![0; builder.candidates.len()]; builder.candidates.len()];
            let locked = vec![vec![Lock::Draw; builder.candidates.len()]; builder.candidates.len()];

            Self {
                candidates: builder.candidates,
                number_of_voters: builder.number_of_voters,
                preferences,
                locked,
                ..Default::default()
            }
        }
    }
}
