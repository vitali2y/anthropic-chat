## Simple Rust CLI chat with Anthropic AI

```shell
 ✗ export ANTHROPIC_API_KEY=XXX  # free from https://console.anthropic.com/settings/workspaces/default/keys , but "Your credit balance is too low ..." might happen
 ✗  # building
 ✗ cargo r --release
~...~
 ✗  # testing
 ✗ ./target/release/anthropic-chat "In Rust what is the difference between Fn, FnMut, and FnOnce?"  # simple prompt as a CLI param
~...~
 ✗ cat my_question.md | ./target/release/anthropic-chat  # prompt as a stdin pipe, e. g. from Markdown file with your code and detailed question
~...~
 ✗
```
