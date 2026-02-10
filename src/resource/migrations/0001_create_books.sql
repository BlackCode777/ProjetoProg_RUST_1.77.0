CREATE TABLE IF NOT EXISTS books (
                                     id UUID PRIMARY KEY,
                                     title TEXT NOT NULL,
                                     author TEXT NOT NULL,
                                     published_year INT,
                                     created_at TIMESTAMPTZ NOT NULL DEFAULT now()
    );
