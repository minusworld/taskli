# Taskli: A very, very small task manager for your CLI.

Taskli lets you do three things:
1. Create tasks
1. Add annotations to tasks
1. Delete tasks

And that's it.

Well, I guess you can view your tasks, too. :)

Taskli is written in Rust.

### Add a task

```bash
$ taskli add "Take care of business"
```

### View tasks

```bash
$ taskli list

 id | description           | created 
----+-----------------------+---------------------------------
 1  | Take care of business | Sun, 12 Jul 2020 01:00:44 +0000 
```

### Add an annotation to a task

```bash
$ taskli annotate 1 "Every day"
$ taskli annotate 1 "Every way"
$ taskli show 1

 id | description           | created                         | annotations 
----+-----------------------+---------------------------------+-------------
 1  | Take care of business | Sun, 12 Jul 2020 01:00:44 +0000 |  Every day  
    |                       |                                 | ----------- 
    |                       |                                 |  Every way   
 ```

 ### Delete a task

 ```bash
 $ taskli delete 1
 $ taskli list

 # No results
 ```

 ## Installation
You'll need the Rust toolchain. Then:

```bash
$ git clone https://github.com/minusworld/taskli
$ cd taskli
$ cargo install
```
