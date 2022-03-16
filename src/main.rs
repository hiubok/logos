// Oliver Kovacs 2022 - logos - MIT

mod json;
use crate::json::*;

static INTRODUCTION_STR: &str =
    "Hello, I'm Logos. I can help you to detect fake news.";
static ENTER_TERM_STR: &str = 
    "Enter a search term!";
static ENTER_NUMBER_STR: &str = 
    "Enter a number to learn more about an item or 'done' to search for new terms!";
static SOURCE_STR: &str =
"

I use the Google Fact Check Tools API:
https://toolbox.google.com/factcheck/apis

It intern uses mulitple credible fact-checking websites to provide data.
The source of a specific result is always given and linked.
";

#[derive(PartialEq)]
enum State {
    Global,
    Claim,
}

struct Bot {
    name: String,
    state: State,
    last_claims: Option<Claims>,
}

impl Bot {
    fn conversation(&mut self) {
        println!("{}: {}\n{}", self.name, INTRODUCTION_STR, ENTER_TERM_STR);
        loop {
            self.respond(self.read());
        }
    }

    fn read(&self) -> String {
        print!("You: ");
        read_line()
    }

    fn preprocess(&self, input: String) -> String {
        input
            .replace(|c: char| !c.is_alphanumeric(), "")
            .to_lowercase()
    }

    fn respond(&mut self, input: String) {
        let input = self.preprocess(input);

        print!("{}: ", self.name);

        if self.is_exit(&input) {
            println!("Goodbye!");
            std::process::exit(0);
        }

        if self.is_source(&input) {
            println!("{}", SOURCE_STR);
            return;
        }

        if self.is_hello(&input) {
            println!("Hello! {}", ENTER_TERM_STR);
            return;
        }

        if self.state == State::Claim {
            self.respond_claim(input);
            return;
        }

        let claims = check(input);
        if let Some(claims_new) = claims.clone().claims {
            self.last_claims = Some(claims);
            self.state = State::Claim;
            self.print_claims(claims_new);
            println!("{}\n", ENTER_NUMBER_STR);
            return;
        }

        println!("Sorry, I couldn't find anything :(");
    }

    fn respond_claim(&mut self, input: String) {
        if input == "done" {
            self.state = State::Global;
            println!("{}", ENTER_TERM_STR);
            return;
        }
        if let Ok(index) = input.parse::<usize>() {
            let index = index - 1;
            let claims = self
                .last_claims
                .as_ref()
                .unwrap()
                .claims
                .as_ref()
                .unwrap();

            if index > claims.len() {
                println!("The number is too big.");
            }
            else {
                self.print_claim(claims[index].clone());
            }
            println!("{}\n", ENTER_NUMBER_STR);
            return;
        }
        println!("Invalid.\n{}\n", ENTER_NUMBER_STR);
    }

    fn is_exit(&self, input: &String) -> bool {
        return match input.as_str() {
            "bye"
            | "goodbye" 
            | "exit"
            | "quit"
            | "q"
            => true,
            _ => false,
        };
    }

    fn is_source(&self, input: &String) -> bool {
        input.contains("source")
    }

    fn is_hello(&self, input: &String) -> bool {
        input == "hi"
            || input == "hii"
            || input == "hey"
            || input == "hello"
            || input == "hallo"
    }

    fn print_claims(&self, claims: Vec<Claim>) {
        println!("\n\nI found the following entries:\n");
        claims
            .iter()
            .enumerate()
            .for_each(|(i, element)| {
                println!(
                    "    {: >2} : {} ({})",
                    i + 1,
                    element.text,
                    element.claim_review[0].textual_rating
                );
            });
        println!();
    }

    fn print_claim(&self, claim: Claim) {
        println!("\n\n{}\n", claim.text);
        if let Some(claimant) = claim.claimant {
            println!("    {: <9}: {}", "Claimant", claimant);
        }
        if let Some(claim_date) = claim.claim_date {
            println!("    {: <9}: {}", "Date", claim_date);
        }
        println!("\nReviews:\n");
        claim.claim_review
            .into_iter()
            .for_each(|review| {
                println!("    {: <15}: {}", "Result", review.textual_rating);
                if let Some(title) = review.title {
                    println!("    {: <15}: {}", "Title", title);
                }
                println!("    {: <15}: {}", "URL", review.url);
                if let Some(name) = review.publisher.name {
                    println!("    {: <15}: {}", "Publisher Name", name);
                }
                if let Some(site) = review.publisher.site {
                    println!("    {: <15}: {}", "Publisher Site", site);
                }
                println!("    {: <15}: {}\n", "Language", review.language_code);
            });
    }
}

impl std::default::Default for Bot {
    fn default() -> Self {
        Bot {
            name: "Bot".to_string(),
            state: State::Global,
            last_claims: None,
        }
    }
}

fn main() {
    let mut bot = Bot::default();
    bot.conversation();
}

fn read_line() -> String {
    let mut out = String::new();
    use std::io::Write;
    std::io::stdout()
        .flush()
        .unwrap();
    std::io::stdin()
        .read_line(&mut out)
        .expect("Did not enter a correct string");
    if let Some('\n') = out.chars().next_back() {
        out.pop();
    }
    if let Some('\r') = out.chars().next_back() {
        out.pop();
    }
    out
}

fn check(query: String) -> Claims {
    let size = 20;
    let key = "AIzaSyA0gsrc5ajXHug_De42hQhxgW9GrxDuZdw";
    let url = format!(
        "https://factchecktools.googleapis.com\
        /v1alpha1/claims:search?query={}&pageSize={}&key={}",
        query, size, key
    );
    let res = reqwest::blocking::get(url)
        .unwrap()
        .text()
        .unwrap();

    serde_json::from_str(&res)
        .expect("invalid response")
}
