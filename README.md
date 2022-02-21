# Remembe.rs

A simple tool to help remind you of things such as links and todo items.

Usage

```
# remembering links (not meant to replace bookmarks..these are links you want to keep in mind to view within the next week)
rmbrs link https://aws.amazon.com/premiumsupport/knowledge-center/rds-mysql-high-replica-lag/
rmbrs link -l # list all links
rmbrs link -s 'replica lag' # search links with "replica lag"..this is the url text itself and can even be the content at the url

# remembering things you wanted to do (does not replace full blown project management tools or to-do trackers)
rmbrs todo review https://github.com/balena-os/balena-supervisor/pull/1805
rmbrs todo -l # list todo items
rmbrs todo -s 'supervisor' # returns all todos with supervisor mentioned in the item text or if todo contains a url searches that resource

# timed/scheduled reminders
rmbrs timer 5d roll out new cluster settings in 5 days # creates a to do that alerts you
rmbrs timer 25/12/2021 wish team merry christmas
rmbrs timer -l # list active alerts waiting to remind you
```
