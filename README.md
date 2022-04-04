# rmb.rs

Closest thing to scribbling stuff on a sticky note 

### Usage

```
~$ rmbrs todos add "finish bandwidth profiles feature"
Added todo
~$ rmbrs todos
[1] work on snowplow
[2] make something in rust
[3] add sentry to the Supervisor
[4] finish bandwidth profiles feature
~$ rmbrs todos rm 2 
Removed todo
~$ rmbrs todos
[1] work on snowplow
[2] add sentry to the Supervisor
[3] finish bandwidth profiles feature
```

### Features
  - [x] Remember todos and links
 
### Goals
  - [ ] Timed todos
    - [ ] Use system notification to notify of a todo 
  - [ ] Ask if you still want to remember a todo/link after X time

### Tricks

I like using this tool by calling the todos in my [zshrc](https://github.com/20k-ultra/dotfiles/blob/b4e3c9219b21c97f0771fa55fc562900cfef9726/zsh/zshrc.symlink#L36). This provides a constant reminder of what I wanted todo without having to ask/check manually.
