
ALTER TABLE contacts ADD CONSTRAINT fk_customer_id
    FOREIGN KEY  (customer_id) REFERENCES "customers"(id)
    ON DELETE CASCADE;

ALTER TABLE lists ADD CONSTRAINT fk_customer_id
    FOREIGN KEY (customer_id) REFERENCES "customers"(id)
    ON DELETE CASCADE;

ALTER TABLE messages
    ADD CONSTRAINT fk_customer_id
        FOREIGN KEY (customer_id) REFERENCES "customers"(id)
        ON DELETE CASCADE,
    ADD CONSTRAINT fk_user_id
        FOREIGN KEY (user_id) REFERENCES "users"(id)
        ON DELETE CASCADE;

ALTER TABLE customer_users ADD CONSTRAINT fk_customer_users_customers
    FOREIGN KEY (customer_id) REFERENCES "customers"(id)
    ON DELETE CASCADE;

ALTER TABLE customer_users ADD CONSTRAINT fk_customer_users_users
    FOREIGN KEY (user_id) REFERENCES "users"(id)
    ON DELETE CASCADE;
