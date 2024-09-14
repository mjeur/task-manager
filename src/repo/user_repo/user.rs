pub struct UserRepository {
    db: Database
}

impl UserRepository {
    pub fn new(db: Database) -> Self {
        UserRepository { db }
    }

    pub fn get_all_users(&self) -> Vec<User> {
        let query = "SELECT * FROM users";
        let rows = self.db.query(query).unwrap();
        let mut users = Vec::new();
        for row in rows {
            let user = User {
                id: row.get("id"),
                name: row.get("name"),
                email: row.get("email"),
            };
            users.push(user);
        }
        users
    }

    pub fn get_user(&self, id: i32) -> Option<User> {
        let query = format!("SELECT * FROM users WHERE id = {}", id);
        let row = self.db.query(query).unwrap().next();
        match row {
            Some(row) => {
                let user = User {
                    id: row.get("id"),
                    name: row.get("name"),
                    email: row.get("email"),
                };
                Some(user)
            }
            None => None,
        }
    }

    pub fn create_user(&self, user: User) -> User {
        let query = format!("INSERT INTO users (name, email) VALUES ('{}', '{}') RETURNING id", user.name, user.email);
        let row = self.db.execute(query).unwrap().next().unwrap();
        let id = row.get("id");
        let user = User {
            id,
            name: user.name,
            email: user.email,
        };
        user
    }

    pub fn update_user(&self, user: User) -> User {
        let query = format!("UPDATE users SET name = '{}', email = '{}' WHERE id = {}", user.name, user.email, user.id);
        self.db.execute(query).unwrap();
        user
    }

    pub fn delete_user(&self, id: i32) -> bool {
        let query = format!("DELETE FROM users WHERE id = {}", id);
        self.db.execute(query).unwrap();
        true
    }
}