# rmb.rs

Command line tool that remembers things for you

Usage

```
# List links
rmbrs links 
# Add a link
rmbrs link https://aws.amazon.com/premiumsupport/knowledge-center/rds-mysql-high-replica-lag/

# List todo items
rmbrs todos
# Add a todo item
rmbrs todo review https://github.com/balena-os/balena-supervisor/pull/1805

# List previously created timers 
rmbrs timers 
# Create a timer that tells you something in 5 days
rmbrs timer 5d "roll out new cluster settings"
# Create a timer with a timestamp
rmbrs timer 25/12/2021 "wish team merry christmas"
```
