# worklog-shell
Worklog's initial repository containing the source code and the executable (for now).

By using the worklog application you can create your own worklog repository which will then contain the worklog binary as well.
All commits to the worklog repository will be handled via GitHub Actions and create a template based table view of your dayly worklog. 

# Usage
Start with `worklog init [name]` to create you own worklog repository. The default name is `worklog`. 
This will create a new folder in the data directory with that name and initialize it as git repository.

Next step is adding a GitHub remote in that newly created repository via `remote add origin pathToYourGitHubRepository` and `git push -u origin main`.

Now you can start the worklog like this:
1. Create a folder in the repository's root with the date you want to add the worklog for like `2023-09-20`
2. Now you can add logs for different projects by creating files like `projectname`. 
Reserved for employees is the name `work` which contains the worklog for what you want to log for your main job.
3. If you want to log learnings you can use the file `learnings`
4. If there were problems or special issues you can log them into `impediments`
5. To log you daily rating and mood you can create a `rating` file
6. For keeping track of the time including breaks without having to add them to the worklog you can add another file called `breaks`
7. If you want to already add a todo (i.e. a day before) you can use `todo.md`
8. To trigger the events for sending a message to teams or jira for a day, add the file `done` (no content needed) to the folder

## File format
Each file needs to be structured in a special way so the parser can parse it correctly.

### projectname, work
This is the most complex, although hopefully straightforward.

start_time
summary
more detailed description
end_time

### learnings, impediments
One learning per line.

__example learnings__
Was surprised how Rust error handling works. Turns out you can just handle the error with `match` and decide if you want to stop the app or continue with or without a message.
And again, planning an app and getting all excited about it is way easier than the implementation. 

__example impediments__
Building the app doesn't work although I used cargo build -r.

### breaks
The format is `hours:minutes`

__example__
lunch: 00:45
shopping: 1:15
kids: 00:30

### rating
mood: good | bad | ok
rating: very good | good | ok | bad | very bad
resume: as planned | mostly as planned | some interruptions | too many interruptions | total chaos

### todo.md
- no parsing right now
- use Markdown, just a bullet list
- like
    - Create repo
    - Push repo
    - Make money


# Dependencies
## Development
- Rust

## Usage
- Git
