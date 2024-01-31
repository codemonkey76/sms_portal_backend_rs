-- root user (at id = 0)
INSERT INTO "users" 
    (id,  typ, username, cid, ctime, mid, mtime) VALUES 
    (0, 'Sys', 'root',  0,   now(), 0,   now());

-- User demo1
INSERT INTO "users" 
    (username, cid, ctime, mid, mtime) VALUES 
    ('demo1',  0,   now(), 0,   now());

-- Customer test-customer (id: 100)
INSERT INTO "customers"    
    (id,  name,            sender_id, is_active, cid, ctime, mid, mtime) VALUES
    (100, 'test customer', 'abc123',  true,      0,   now(), 0,   now());

INSERT INTO "customers"    
    (id,  name,            sender_id, is_active, cid, ctime, mid, mtime) VALUES
    (101, 'test customer 2', 'abc123',  true,      0,   now(), 0,   now());
