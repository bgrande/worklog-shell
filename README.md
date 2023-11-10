# Table of contents
1. [About](#about)
2. [Usage](#usage)
  1. [File Format](#format)
     1. [Work, Project](#work)
     2. [Learnings, Impediments](#learning)
     3. [Breaks](#breaks)
     4. [Rating](#rating)
     5. [Todo](#todo)
3. [Dependencies](#dependencies)
  1. [Development](#deps-dev)
  2. [Usage](#deps-usage)
4. [Goals](#goals)
5. [Roadmap](#roadmap)


# About <a name="about"></a>
Worklog's initial repository containing the source code and the executable (for now).

By using the worklog application you can create your own worklog repository which will then contain the worklog binary as well.
All commits to the worklog repository will be handled via GitHub Actions and create a template based table view of your dayly worklog. 

# Usage <a name="usage"></a>
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

## File format <a name="format"></a>
Each file needs to be structured in a special way so the parser can parse it correctly.

### projectname, work <a name="work"></a>
This is the most complex, although hopefully straightforward.

> start_time<br>
> summary; summary 2; summary 3<br>
> more detailed description<br>
> end_time<br>
> start_time2<br>
> summary4; summary5<br>
> JIRA-1: more details<br>
> extended details<br>
> end_time2<br>

When using a jira taskId like `PROJECT-1: my work done` this will be added as a jira comment 
when a jira hook is active and a token exists. 

### learnings, impediments <a name="learning"></a>
One learning per line.

__example learnings__

> Was surprised how Rust error handling works. Turns out you can just handle the error with `match` and decide if you want to stop the app or continue with or without a message.<br>
And again, planning an app and getting all excited about it is way easier than the implementation. 

__example impediments__

> Building the app doesn't work, although I used cargo build -r.

### breaks <a name="breaks"></a>
The format is `hours:minutes`

__example__

> lunch: 00:45<br>
> shopping: 1:15<br>
> kids: 00:30

### rating <a name="rating"></a>
Daily rating, like your mood, rating of the day and if it worked out like planned (or not).
> mood: good | bad | ok<br>
> rating: very good | good | ok | bad | very bad<br>
> structure: as planned | mostly as planned | some interruptions | too many interruptions | total chaos

### todo.md <a name="todo"></a>
- no parsing right now
- use Markdown, with the project name as heading and just a bullet list
- like
```
# worklog
- Create repo
- Push repo
- Make money
```

See the folder [example](/example) for more detailed examples.


# Dependencies <a name="dependencies"></a>
## Development <a name="deps-dev"></a>
- Rust

## Usage <a name="deps-usage"></a>
- Git
                       

# Goals <a name="goals"></a>
1. Learning more about Rust and start with using nom
2. Train my Rust skills
3. Make keeping track of my work a bit easier (right now I'm using spreadsheets for each day)
4. Automating sending work messages to work related channels 

# Roadmap <a name="roadmap"></a>
## 0.2.0
- add rating parser
- add impediment/learning parser
- add project parser
- add breaks parser (similar to rating)

## 0.1.0
+ create structure
+ setup repository logic