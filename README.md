# Submission Guide

## How to ( not ) submit final assignment tasks

Here you will find short ( or not so short ) instructions on how to send tasks.

### Summary

- Read your condition **carefully** and even a few times.
- Create a new project with `cargo new --lib solution`
- Write a solution in `src/lib.rs`
- **Perform the sample test.**
    If your code is not compiled on the sample test, you will receive 0 points.
- Check that the types of functions and structures are the same ones we have described in the condition. If you decide to return type `Foo&Foo`instead of the type described, our tests just won't compile and you'll get 0 points.
- Do not use external libraries ( you cannot upload them to the ) interface.
- Do not share your decision with anyone.
- Don't watch other people's decisions.
- Copy the entire content to `src/lib.rs`in the text box on the site.

## Honor Code

### sharing of solutions

All decisions become public after the deadline. Before that, any sharing decisions are prohibited.

This includes:

- Send your decision to someone else.
- Show it to a friend to "eat ideas".
- Put it somewhere public before the ( deadline, for example GitHub ).

If we catch that you copied, you will lose all your points from tasks. This includes submissions that have been changed to look different. This it's easier to catch than you think.

On the other hand, don't hesitate to write the simplest possible code because "other people will pass on the same decision". It is also easily recognizable, and of course, we will not assume that you copied. Write the best code you can, don't share your decisions and there will be no reason to worry that we will think, that you copied.

## What do you need

If you are sending a task for the first time, check that you have a cargo:

```
$ cargo --version
cargo 1.64.0 (387270bc7 2022-09-16)
```

The testing is done using the command `cargo test`.

## Sample

Fpr each task there are example tests. Mandatory **download the test and run it for eacah test**. Tasks are automatically checked with tests, and if your code is not compiled with the base test, it will not be compiled during the check. As a result of which you will get 0 points.

If the base test is compiled but not passed, you do not appear to have implemented something right - you can still run a homework for which you can earn a partial number of points. However, we strongly advise you to just start earlier with homework and decide it completely.

To run the test:

- Save the test file as `tests/test_basic.rs` (in the project initialized above with `cargo new`)
- Follow the following command:
    ```
    $ cargo test
    ```
For more information on testing in Rust, read [this chapter in the documentation](https://doc.rust-lang.org/book/ch11-01-writing-tests.html).

## More tips

- Feel completely free to write more tests. The sample test file will be only sanity check, which checks a few simple cases. The real test will be much more thorough and it is good to consider what other cases are not covered, and add them.
- The first homework may seem easy to you. This can make you think, "Well, I can do that to write it in an hour or two before the deadline". You probably can't. Rust is not an easy language to learn, and if you haven't already written a Rust code, don't underestimate the difficulty of the first homework, even if you the condition seems easy.
- The second homework you might think will be as easy as the first and leave it for an hour or two before the deadline. Will not be. When we get to the second homework, we will have enough interesting material to challenge you a little.
- In general, do not leave your homework for the last moment. If nothing else, if you run your homework for half an hour before the deadline with some small error, there is no time to see this and warn you. A message from the "I was just about to fix it, but I was 1 minute late after the deadline" will have no effect - 1 minute around the deadline is not the time to write or fix your homework, considering we'll give you one whole week.
