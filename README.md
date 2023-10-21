# Introduction
I really enjoy the [3-2-1](https://jamesclear.com/3-2-1) newsletter from [James Clear](https://jamesclear.com/). Especially, the questions that come at the end. It has often helped me take a minute and think about things at a macro level. This repository is a simple rust program that scraps the newsletter and extracts these questions to a file. 

## Update
Huge thanks to [feedback](https://gist.github.com/kpcyrd/7342c3f833fbd09e98a765ca8417922e) from [kpcyrd](https://github.com/kpcyrd). I learned about the [anyhow](https://docs.rs/anyhow/latest/anyhow/) crate from them and have incorporated the suggested changes.

# Instructions
The following commands can be used to generate the *questions* file:
```
git clone https://github.com/unbeschwert/3-2-1.git questions
cd questions 
cargo build
./target/debug/james-clear-3-2-1
```

The file can be found at ```$HOME/questions```.

Further in Linux, one can do the following: Add ```alias question="cat $HOME/questions | shuf | head -1"``` to ```.bash_aliases``` and enjoy pondering about the questions.

# Note
This program is tested on Ubuntu. If you have any questions or suggestions, please do create an issue [here](https://github.com/unbeschwert/3-2-1/issues).
