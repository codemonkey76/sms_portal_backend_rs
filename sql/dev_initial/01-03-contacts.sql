CREATE TABLE contacts (
    -- PK
    id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,

    phone varchar(255) NOT NULL,
    first_name varchar(255) NOT NULL,
    last_name varchar(255) NOT NULL,
    company_name varchar(255) NOT NULL,
  
    -- FK
    customer_id BIGINT NOT NULL

    -- Timestamps
    cid bigint NOT NULL,
    ctime timestamp with time zone NOT NULL,
    mid bigint NOT NULL,
    mtime timestamp with time zone NOT NULL
);