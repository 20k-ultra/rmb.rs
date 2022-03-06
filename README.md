# rmb.rs

Closest thing to scribbling stuff on a sticky note 

Usage

```
# List links
rmbrs links 
# Add a link
rmbrs links add https://aws.amazon.com/premiumsupport/knowledge-center/rds-mysql-high-replica-lag/
# Remove a link (index of link in list of links)
rmbrs links rm 1 

# List todo items
rmbrs todos
# Add a todo item
rmbrs todos add "review https://github.com/balena-os/balena-supervisor/pull/1805"
# Remove a todo (index of todo in list of todos)
rmbrs todos rm 1 

# List previously created timers 
rmbrs timers 
# Create a timer that tells you something in 5 days
rmbrs timers add "roll out new cluster settings" 5d
# Create a timer with a timestamp
rmbrs timers add "wish team merry christmas" 25/12/2021 
# Remove a timer (index of todo in list of timers)
rmbrs timers rm 1 
```
