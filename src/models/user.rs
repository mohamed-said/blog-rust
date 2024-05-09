use bson::{doc, oid::ObjectId, Document};
use mongodb::Database;
use serde::{Deserialize, Serialize};

use crate::error::user_error::{Result, UserError};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    _id: String,
    name: String,
    email: String,
    phone: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddUserRequest {
    name: String,
    email: String,
    phone: String,
}

#[derive(Clone)]
pub struct UserController {
    db_instance: Database,
}

impl UserController {
    pub fn new(db_instance: Database) -> UserController {
        UserController { db_instance }
    }

    // TODO handle duplicate users
    pub async fn create_user(&self, req: AddUserRequest) -> Result<ObjectId> {
        // FIXME make the collection name a constant
        let collection = self.db_instance.collection::<Document>("users");

        let record = collection
            .insert_one(
                doc! {
                    "name": req.name,
                    "email": req.email,
                    "phone": req.phone,
                },
                None,
            )
            .await;

        let inserted_id_bson = match record {
            Ok(r) => r.inserted_id,
            Err(e) => {
                println!("Error: Insert one item failed: {:?}", e);
                return Err(UserError::UserNotAdded);
            }
        };

        match inserted_id_bson.as_object_id() {
            Some(id) => Ok(id),
            None => {
                // FIXME handle log errors better
                println!("Error: Parsing objectId failed!");
                Err(UserError::UserNotAdded)
            }
        }
    }
}
