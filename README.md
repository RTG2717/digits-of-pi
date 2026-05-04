# digits-of-pi

Just calculating digits of pi!! (as normal human beings do)

## Some Info

If I simply divide 22/7 then it will be limited by the limit of f64 (i.e ~15-17 digits as per GPT), so instead I am adding a simple division logic, and appending each digit to a txt file to be readable.

> The `output/Output.AutoUpdated.txt` file will be pushed to git every 6 hours.

## TODO

- [x] Initial Functionality
- [] Make all file operations atomic (eg: create a tmp file instead of directly overwriting the remainder file, think of something for output file too)
- [] Automate code updation *to/from* git
- [] Automate code continuation from update *from* git
- [] Improve Error Logging

