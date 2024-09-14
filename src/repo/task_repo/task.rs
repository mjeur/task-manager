use crate::database::Database;
use crate::models::Task;

pub struct TaskRepository {
    db: Database
}


impl TaskRepository {
    pub fn new(db: Database) -> Self {
        TaskRepository { db }
    }

    pub fn get_all_tasks(&self) -> Vec<Task> {
        let query = "SELECT * FROM tasks";
        let rows = self.db.query(query).unwrap();
        let mut tasks = Vec::new();
        for row in rows {
            let task = Task {
                id: row.get("id"),
                author_id: row.get("author_id"),
                title: row.get("title"),
                description: row.get("description"),
                project_id: row.get("project_id"),
            };
            tasks.push(task);
        }
        tasks
    }

    pub fn get_task(&self, id: i32) -> Option<Task> {
        let query = format!("SELECT * FROM tasks WHERE id = {}", id);
        let row = self.db.query(query).unwrap().next();

        match row {
            Some(row) => {
                let task = Task {
                    id: row.get("id"),
                    author_id: row.get("author_id"),
                    title: row.get("title"),
                    description: row.get("description"),
                    project_id: row.get("project_id"),
                };
                Some(task)
            }
            None => None,
        }
    }

    pub fn create_task(&self, task: Task) -> Task {
        let query = format!("INSERT INTO tasks (title, description, project_id) 
            VALUES ('{}', '{}', {}) RETURNING id", 
            task.title, task.description, task.project_id);
        let row = self.db.execute(query).unwrap().next().unwrap();
        let id = row.get("id");
        //let id = self.db.last_insert_id().unwrap();
        let task = Task {
            id,
            author_id: row.get("author_id"),
            title: task.title,
            description: task.description,
            project_id: task.project_id,
        };
        task
    }

    pub fn update_task(&self, task: Task) -> Task {
        let query = format!("UPDATE task SET author_id = {}, title = '{}', 
            description = '{}',  project_id = {} WHERE id = {}", 
                task.author_id, task.title, task.description, task.project_id, task.id);

        self.db.execute(query).unwrap();
        task
    }

    pub fn delete_task(&self, id: i32) -> bool {
        let query = format!("DELETE FROM tasks WHERE id = {}", id);
        self.db.execute(query).unwrap();
        true
    }
}