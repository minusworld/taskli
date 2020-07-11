use chrono::{Utc, DateTime};
use clap::{value_t, App, Arg, AppSettings};
use rusqlite::{params, Connection, NO_PARAMS};
#[macro_use] extern crate prettytable;
use prettytable::{format, Table};

struct Task {
    id: Option<i64>,
    description: String,
    created: DateTime<Utc>,
    annotations: Vec<String>,
}

fn add_task(conn: &Connection, task: Task) -> Result<(), rusqlite::Error> {
    let created = task.created.to_rfc2822();
    conn.execute(
        "INSERT INTO tasks (description, created)
        VALUES (?1, ?2);",
        params![task.description, created]
    )?;

    // TODO: This is borked.
    let row_id = conn.last_insert_rowid();
    for annotation in task.annotations.iter() {
        add_annotation(conn, row_id, annotation)?;
    }

    Ok(())
}

fn add_annotation(conn: &Connection, task_id: i64, description: &String) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO annotations (task_id, description)
        VALUES (?1, ?2);",
        params![task_id, description]
    )?;

    Ok(())
}

fn delete_task(conn: &Connection, task_id: i64) -> Result<(), rusqlite::Error> {
    conn.execute(
        "DELETE FROM tasks WHERE id = ?1;",
        params![task_id]
    )?;

    Ok(())
}

fn show_task_list(conn: &Connection) -> Result<(), rusqlite::Error> {
    let mut statement = conn.prepare("SELECT * FROM tasks")?;

    let task_iterator = statement.query_map(NO_PARAMS, |row| {
        let created: String = row.get(2)?;
        Ok(Task {
            id: row.get(0)?,
            description: row.get(1)?,
            created: DateTime::parse_from_rfc2822(&created).unwrap().with_timezone(&Utc),
            annotations: vec![]
        })
    })?;

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(row!["id", "description", "created"]);
    for taskr in task_iterator {
        let task = taskr.unwrap();
        let id = task.id.unwrap();

        table.add_row(row![id, task.description, task.created.to_rfc2822()]);
    }

    table.printstd();

    Ok(())
}

fn show_task_details(conn: &Connection, task_id: i64) -> Result<(), rusqlite::Error> {
    let mut statement = conn.prepare("SELECT * FROM tasks WHERE id = ?")?;

    let task_iterator = statement.query_map(params![task_id], |row| {
        let created: String = row.get(2)?;
        Ok(Task {
            id: row.get(0)?,
            description: row.get(1)?,
            created: DateTime::parse_from_rfc2822(&created).unwrap().with_timezone(&Utc),
            annotations: vec![]
        })
    })?;

    let mut annotation_statement = conn.prepare("SELECT * FROM annotations WHERE task_id = ?")?;

    let annotation_iterator = annotation_statement.query_map(params![task_id], |row| {
        let annotation: String = row.get(2)?;
        Ok(annotation)
    })?;

    let mut annotation_table = Table::new();
    annotation_table.set_format(*format::consts::FORMAT_NO_BORDER);
    for annotationr in annotation_iterator {
        let annotation = annotationr.unwrap();
        annotation_table.add_row(row![annotation]);
    }

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(row!["id", "description", "created", "annotations"]);
    for taskr in task_iterator {
        let task = taskr.unwrap();
        let id = task.id.unwrap();

        table.add_row(row![id, task.description, task.created.to_rfc2822(), annotation_table]);
    }

    table.printstd();

    Ok(())
}

fn initialize() -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open("test.db")?;

    // Create tasks table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id      INTEGER PRIMARY KEY,
            description TEXT NOT NULL,
            created TEXT
        );",
        NO_PARAMS,
    )?;

    // Create annotations table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS annotations (
            id      INTEGER PRIMARY KEY,
            task_id INTEGER NOT NULL,
            description TEXT NOT NULL,
            FOREIGN KEY(task_id) REFERENCES tasks(id) ON DELETE CASCADE
        )",
        NO_PARAMS,
    )?;

    Ok(conn)
}

fn main() -> () {
    let version = "0.0.1";
    let author = "minusworld";

    let app = App::new("taskr")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(version)
        .author(author)
        .subcommand(
            App::new("add")
                .about("Add a new task")
                .arg(
                    Arg::with_name("description")
                        .help("A description for the task")
                        .index(1)
                        .required(true)
                )
        ).subcommand(
            App::new("delete")
                .about("Delete a task")
                .arg(
                    Arg::with_name("task_id")
                        .help("The ID of the task to delete")
                        .required(true)
                        .index(1)
                )
        ).subcommand(
            App::new("list")
                .about("List all tasks")
        ).subcommand(
            App::new("show")
                .about("Display details about a task")
                .arg(
                    Arg::with_name("task_id")
                        .help("The ID of the task to show")
                        .index(1)
                        .required(true)
                )
        ).subcommand(
            App::new("annotate")
                .about("Add an annotation to a task")
                .arg(
                    Arg::with_name("task_id")
                        .help("The ID of the task to annotate")
                        .index(1)
                        .required(true)
                )
                .arg(
                    Arg::with_name("annotation")
                        .help("Annotation to append to a task")
                        .index(2)
                        .required(true)
                )
        );

        let matches = app.get_matches();

        let conn = initialize().expect("Could not initialize database.");

        match matches.subcommand() {
            ("add", Some(submatches)) => {
                let task = Task {
                    id: None,
                    description: String::from(submatches.value_of("description").unwrap()),
                    created: Utc::now(),
                    annotations: vec![],
                };
                add_task(&conn, task).expect("Could not add task.");
            },
            ("delete", Some(submatches)) => {
                delete_task(&conn, value_t!(submatches.value_of("task_id"), i64).unwrap()).expect("Could not delete task.");
            },
            ("list", Some(_submatches)) => {
                show_task_list(&conn).expect("Could not show task list.");
            },
            ("show", Some(submatches)) => {
                show_task_details(&conn, value_t!(submatches.value_of("task_id"), i64).unwrap())
                    .expect("Could not get task details.");
            },
            ("annotate", Some(submatches)) => {
                add_annotation(
                    &conn,
                    value_t!(submatches.value_of("task_id"), i64).unwrap(),
                    &String::from(submatches.value_of("annotation").unwrap())
                ).expect("Could not annotate task.");
            },
            _ => ()
        }
}