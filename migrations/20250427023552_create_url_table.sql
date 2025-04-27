CREATE TABLE
    url (
        id SERIAL PRIMARY KEY,
        short TEXT UNIQUE NOT NULL,
        original TEXT NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT now ()
    );