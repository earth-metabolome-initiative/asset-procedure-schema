CREATE TABLE brands (
    id UUID PRIMARY KEY REFERENCES ownables(id) ON DELETE CASCADE,
    name TEXT NOT NULL CHECK (
        name <> ''
        AND length(name) <= 255
    )
);