#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use std::sync::Mutex;
use std::collections::HashMap;

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

type ID = String;
type DocMap = HashMap<String, String>;
type FlatCache = Mutex<HashMap<ID, DocMap>>;

#[derive(Serialize, Deserialize)]
struct Message {
    subdocid: String,
    contents: String
}

#[derive(Serialize, Deserialize)]
struct Document {
    contents: Vec<Message>
}

#[post("/<id>", format = "json", data = "<message>")]
fn new(id: ID, message: Json<Message>, context: State<catcore>) -> JsonValue {
    context.put_value(id, message);
    json!({ "status": "ok" })
}

#[get("/<id>/<subdoc_id>", format = "json")]
fn get_value(id: ID, subdoc_id: String, context: State<catcore>) -> JsonValue {
    context.get_value(id, subdoc_id)
}

#[get("/<id>")]
fn get_document(id: ID, context: State<catcore>) -> JsonValue {
    json!(context.get_aggregate_value(id))
}

#[delete("/<id>/<subdoc_id>", format = "json")]
fn delete_value(id: ID, subdoc_id: String, context: State<catcore>) -> JsonValue {
    context.delete_subdoc(id, subdoc_id)
}

#[delete("/<id>")]
fn delete_doc(id: ID, context: State<catcore>) -> JsonValue {
    context.delete_doc(id)
}


struct catcore
{
    cache: FlatCache
}

impl catcore
{
    pub fn new() -> Self
    {
        let docmap = HashMap::<ID, DocMap>::new();
        catcore {
            cache: Mutex::new(docmap)
        }
    }

    pub fn put_value(&self, id: ID, message: Json<Message>)
    {
        let mut cache = self.cache.lock().expect("Failed to lock cache");
        let docid = message.0.subdocid;
           
        if let Some(doc_entry) = cache.get_mut(&id)
        {
            doc_entry.insert(docid, message.0.contents);
        }
        else
        {
            let mut new_entry = DocMap::new();
            new_entry.insert(docid, message.0.contents);
            cache.insert(id, new_entry);

        }        
    }

    pub fn get_value(&self, id: ID, subdoc_id: String) -> JsonValue
    {
        let cache = self.cache.lock().expect("failed to lock cache");
        if let Some(doc_entry) = cache.get(&id)
        {
            if let Some(content) = doc_entry.get(&subdoc_id)
            {
                return json!({"status": "ok", "content" : content});
            }
            else
            {
                return json!({"status" : "unknown subdoc ID!"})
            }
        }
        else
        {
            return json!({"status" : "unknown ID"})
        }
    }

    pub fn get_aggregate_value(&self, id: ID) -> Document
    {
        let cache = self.cache.lock().expect("failed to lock cache");
        let mut doc = Document{contents: vec![]};
        if let Some(doc_entry) = cache.get(&id)
        {
            for (key, val) in doc_entry.iter()
            {
                let subdoc = Message{subdocid: key.clone(), contents: val.clone()};
                doc.contents.push(subdoc);
                
            }
        }
        return doc
    }

    pub fn delete_subdoc(&self, id: ID, subdoc_id: String) -> JsonValue
    {
        let mut cache = self.cache.lock().expect("failed to lock cache");
        if let Some(doc_entry) = cache.get_mut(&id)
        {            
            if let Some(_) = doc_entry.get(&subdoc_id)
            {
                doc_entry.remove_entry(&subdoc_id);
                return json!({"status": "ok"});
            }
            else
            {
                return json!({"status" : "unknown subdoc ID"})
            }
        }
        else
        {
            return json!({"status" : "unknown ID"})
        }
    }

    pub fn delete_doc(&self, id: ID) -> JsonValue
    {
        let mut cache = self.cache.lock().expect("failed to lock cache");
        if let Some(_) = cache.get(&id)
        {            
            cache.remove_entry(&id);
            return json!({"status": "ok"});
        }
        else
        {
            return json!({"status" : "unknown ID"})
        }
    }
}

pub fn launch()
{
    rocket::ignite()
    .mount("/", routes![new, get_value, delete_value, delete_doc, get_document])
    .manage(catcore::new())
    .launch();
}
