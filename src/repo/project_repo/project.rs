pub struct ProjectRepository {
    db: Database
}

impl ProjectRepository {
    pub fn new(db: Database) -> Self {
        ProjectRepository { db }
    }

    pub fn get_all_projects(&self) -> Vec<Project> {
        let query = "SELECT * FROM projects";
        let rows = self.db.query(query).unwrap();
        let mut projects = Vec::new();
        for row in rows {
            let project = Project {
                id: row.get("id"),
                author_id: row.get("author_id"),
                name: row.get("name"),
            };
            projects.push(project);
        }
        projects
    }

    pub fn get_all_projects_with_tasks(&self) -> Vec<Project> {
        let query = "
            SELECT p.id as project_id, p.author_id as project_author_id, p.name as project_name,
                   t.id as task_id, t.author_id as task_author_id, t.title as task_title, 
                   t.description as task_description, t.project_id as task_project_id
            FROM projects p
            LEFT JOIN tasks t ON p.id = t.project_id
        ";
        
        let rows = self.db.query(query).unwrap();
        let mut projects_map: HashMap<i32, DbProject> = HashMap::new();

        for row in rows {
            let project_id: i32 = row.get("project_id");
            let project_name: String = row.get("project_name");
            let project_author_id: i32 = row.get("project_author_id");

            let task_id: Option<i32> = row.get("task_id");
            let task = if let Some(task_id) = task_id {
                Some(Task {
                    id: task_id,
                    author_id: row.get("task_author_id"),
                    title: row.get("task_title"),
                    description: row.get("task_description"),
                    project_id: row.get("task_project_id"),
                })
            } else {
                None
            };

            projects_map.entry(project_id)
                .and_modify(|project| {
                    if let Some(task) = task.clone() {
                        project.tasks.push(task);
                    }
                })
                .or_insert_with(|| DbProject {
                    id: project_id,
                    author_id: project_author_id,
                    name: project_name,
                    tasks: task.map_or_else(Vec::new, |t| vec![t]),
                });
        }
        projects_map.into_values().collect()
    }

    pub fn get_project(&self, id: i32) -> Option<Project> {
        let query = format!("SELECT * FROM projects WHERE id = {}", id);
        let row = self.db.query(query).unwrap().next();
        match row {
            Some(row) => {
                let project = Project {
                    id: row.get("id"),
                    name: row.get("name"),
                };
                Some(project)
            }
            None => None,
        }
    }

    pub fn create_project(&self, project: Project) -> Project {
        let query = format!("INSERT INTO projects (name) VALUES ('{}') RETURNING id", project.name);
        let row = self.db.execute(query).unwrap().next().unwrap();
        let id = row.get("id");
        let project = Project {
            id,
            name: project.name,
        };
        project
    }

    pub fn update_project(&self, project: Project) -> Project {
        let query = format!("UPDATE projects SET name = '{}' WHERE id = {}", project.name, project.id);
        self.db.execute(query).unwrap();
        project
    }

    pub fn delete_project(&self, id: i32) -> bool {
        let query = format!("DELETE FROM projects WHERE id = {}", id);
        self.db.execute(query).unwrap();
        true
    }
}