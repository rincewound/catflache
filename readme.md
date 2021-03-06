# Catflache
A deadsimple document store

## WTF is a "Catflache"
Catflache is intended to be a simple in-memory datastore to share data & documents between applications running on different machines. It provides a REST-like API for storing and retrieving documents from the store.

## Limitations
Catflache is RAM backed only. There is no failover, and *no* limit to the amount of RAM it will use, thus - if used carelessly - it will gobble up as much RAM as available on the hosting system.

## Datastore organisation
Catflach uses a double-id concept, where each document can have an arbitrary number of subdocuments. Documents and Subdocuments both have an id. A given subdocument is uniquely identified by its combination of id and subdoc_id. Ids must be unique (not that Catflache will silently update a document if an Id clash occurs), subdoc_ids must be unique within the namespace generated by the document to which they are associated, i.e. it is permitted to have a subdoc called "foobar" in document "barkbark" and "borkbork". Catflache will silently update a subdoc if subdoc_id clash occurs.

## API

POST /\<id>

Puts new data into the document store or updates existing data. Both documents and subdocuments will be generated if they are not present. This call expects a json body of the form:

```
{
    "subdocid": "subdoc_id_value",
    "content": "document contents"
}
```

GET /\<id>/\<subdoc_id>

Retrieves the content of a given subdoc. The call will return a json body of the form:

```
{
    "status" : "ok"|"unknown subdoc ID"| "unknown ID"
    "content": "document contents"
}
```

Note that the "content" part of the response will only be present in case the status is "ok"


DELETE /\<id>
Deletes a document and all subdocuments with a given id. Will return a json body of the form:

```
{
    "status" : "ok"|"unknown ID"
}
```

DELETE /\<id>/\<subdoc_id>
Deletes a subdocument within the specified document.

```
{
    "status" : "ok"|"unknown ID|unkown subdoc ID"
}
```

## Usage Examples
### Python
This example assumes you have "requests" installed previously.

Retrieving a document:
```
import requests
r = requests.get("http://localhost:8000/1234")

print(r.content)
```

Publishing a document:

```
import requests
r = requests.post("http://localhost:8000/1234", json={"subdocid": "1250", "contents": "This is a test"})
```

## Configuration
As catflache is a Rocket.rs application, you can use the checked-in Rocket.toml file to configure the application, e.g. to change ports or IP adresses to which the server should bind. 

## Acknowledgements
Catflache is a "code-done-quick" project and uses a ton of readily available libs, most notable Rocket.rs. It was also basically derived from a few hunks of Rocket's example code