# Introduction
I really enjoy the [3-2-1](https://jamesclear.com/3-2-1) newsletter from [James Clear](https://jamesclear.com/). Especially, the questions that come at the end. It has often helped me take a minute and think. This repository is a simple rust program that scraps the newsletter and extracts these questions to a file. 

# Instructions
The following commands can be used to generate the *questions* file:
```
git clone https://github.com/unbeschwert/3-2-1.git questions
cd questions 
cargo build
./target/debug/james-clear-3-2-1
```

The file is can be found $HOME/questions.

Further in Linux, one can do the following:
Add ```alias questions="cat $HOME/questions | shuf | head -1"``` to .bash_alias and enjoy pondering about the questions.

# Note
This program is tested on Ubuntu. If you have any questions or suggestions, please do create an issue [here](https://github.com/unbeschwert/3-2-1/issues).
