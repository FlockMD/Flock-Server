
Top-level system design overview:

Databases:

    - Document Storage: Mongo (or some other document store)
        --> Since documents will be stored as JSON files, want some non-relational/document storage
       
Server Architecture:
    - Load balancer --> Application Server Instances
        - Once a document has at least one connection, all other users trying to connect to that document at the same time must be routed to that instance of the server (unless we design a distributed message queue)
    - Application Server:
        - Custom implementation of message queue which handles:
            --> Establishing WebSocket connection to actual document
            --> Serving reads and broadcasting incoming writes to all connected users
            --> Writing incoming changes





Design Ideas:

Maintain, per editor, a list of nodes that they created

Two approaches to ID these:



Idea 1: Assign all users a 'username' (string) id. Assign specific type for guest user



Then, once all guest user websocket connections disconnect, merge all guest user Vectors into one central guest vector in order to free old guest ids
--> For signed in users, maintain a vector per, which will allow old users to view previous revision history

NOTE: per document, probably cannot reuse old ids of deleted nodes. Otherwise, consider the following data race:
    --> Users A and B
    --> Most recently typed node Ax (x is the numerical part of the ID, A for user A)
    --> User B attempts to delete Ax, at the same time
   
    NVM PROB NOT A PROBLEM
   

Saving/Writing Document to DB:

Two basic approaches:

    1. Easy approach: Manual saving initialized by user, writes entire document to db at once
    2.  
        --> Store a commit log with all changes since last save
        --> Once enough changes have been made (when users stop typing for X amount of time, enough chars have been written, etc.), write through to db (either entire document, or just changes)



Also need a smarter approach to Serializing document (as plaintext/text that user will see in editor)
    --> If we call render(doc) clientside on every change, for a large document performance will suffer
