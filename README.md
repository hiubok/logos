# Logos

[![Run on Repl.it](https://repl.it/badge/github/hiubok/logos)](https://repl.it/@OliverKovacs/logos#README.md)

Logos is a chatbot that can fact-check news for you.

Logos uses a crude custom Algorithm for natural language processing (NLP) and [Google Fact Check Tools API](https://developers.google.com/fact-check/tools/api) for fact checking.  
It is written in [Rust](https://www.rust-lang.org/) and can be run online in Repl or locally.

This project was made for the [2022 #4GoodAI hackathon](https://www.ada.wien/hackathon-4-good-ai/hackathon-4goodai-2022/).

If you have any questions, problems or suggestions write us an [email](#authors) or open a [GitHub issue](https://github.com/hiubok/logos/issues).

**[Demo](https://www.youtube.com/watch?v=oo2_16VFMwo)** (YouTube)

## Table of contents

- [Usage](#usage)
- [Languages and Technologies](#languages-and-technologies)
- [Sources](#sources)
- [Authors](#authors)
- [Disclaimer](#disclaimer)
- [Legal notice](#legal-notice)

## Usage

### Repl
Go to https://repl.it/@OliverKovacs/logos#README.md  
Click `Run`

### Locally
You need [Rust](https://www.rust-lang.org/) installed for this.  
Clone the repo with [git](https://git-scm.com/)
```bash
git clone https://github.com/hiubok/logos && cd logos
```
or download as a [zip](https://github.com/hiubok/logos/archive/main.zip).

In the directory run the command
```bash
cargo run
```

## Languages and technologies
The bot is written in [Rust](https://www.rust-lang.org/).
We used these technologies to create the bot (click on an icon for more info):

[<img align="left" alt="Rust" width="28px" src="./assets/rust.svg" />](https://www.rust-lang.org/)
[<img align="left" alt="Google Fact Check" width="28px" src="./assets/factcheck.png" />](https://developers.google.com/fact-check/tools/api)
[<img align="left" alt="Google Cloud" width="28px" src="./assets/googlecloud.svg" />](https://cloud.google.com/)
[<img align="left" alt="git" width="28px" src="./assets/git.svg" />](https://git-scm.com/)
[<img align="left" alt="GitHub" width="28px" src="./assets/github.svg" />](https://github.com/)
[<img align="left" alt="Repl" width="28px" src="./assets/repl.png" />](https://repl.it/)
[<img align="left" alt="Visual Studio Code" width="28px" src="./assets/vscode.svg" />](https://code.visualstudio.com/)

&nbsp;

## Sources
Logos uses the [Google Fact Check Tools API](https://developers.google.com/fact-check/tools/api) for getting information, which in turn relies on various other fact-checking websites.

## Authors
- Oliver Kovacs
    - [Github](https://github.com/OliverKovacs)
    - [Email](mailto:oliver.kovacs.dev@gmail.com)
- Ulrich Barnstedt

## Disclaimer
Do NOT rely on the data provided by the bot for real world use, this is a work in progress, proof-of-concept project.

## Legal Notice

By using this program you agree to:
- Google’s API [Terms of Service](https://developers.google.com/terms/)
