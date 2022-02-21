# rmb.rs

A simple tool to help remind you of things such as links and todo items.

Usage

```
# List links
rmbrs link # list all links
# Add a link
rmbrs link https://aws.amazon.com/premiumsupport/knowledge-center/rds-mysql-high-replica-lag/

# List todo items
rmbrs todo
# Add a todo item
rmbrs todo review https://github.com/balena-os/balena-supervisor/pull/1805

# List active reminders that will fire in the future
rmbrs timer 
# Create a timer that tells you something in 5 days
rmbrs timer 5d "roll out new cluster settings"
# Create a timer with a timestamp
rmbrs timer 25/12/2021 "wish team merry christmas"
```
