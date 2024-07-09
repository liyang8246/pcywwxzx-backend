use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub enum Permission {
    Admin,
    Root,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct User {
    pub id: Option<String>,
    pub password: Option<String>,

    pub name: Option<String>,
    pub class: Option<String>,
    pub phone: Option<String>,
    pub wecheat: Option<String>,
    pub permission: Option<Permission>,
}

impl User {
    fn new() -> Self {
        User {
            id: None,
            password: None,
            name: None,
            class: None,
            phone: None,
            wecheat: None,
            permission: None,
        }
    }

    fn id(mut self,id:String) -> Self {
        self.id = Some(id);
        self
    }

    fn password(mut self,password:String) -> Self {
        self.password = Some(password);
        self
    }

    fn name(mut self,name:String) -> Self {
        self.name = Some(name);
        self
    }

    fn phone(mut self,phone:String) -> Self {
        self.phone = Some(phone);
        self
    }

    fn wecheat(mut self,wecheat:String) -> Self {
        self.wecheat = Some(wecheat);
        self
    }

    fn class(mut self,class:String) -> Self {
        self.class = Some(class);
        self
    }

    fn permission(mut self,permission:Permission) -> Self {
        self.permission = Some(permission);
        self
    }
}
