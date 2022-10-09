CREATE SCHEMA rsvp;
-- AWS RDS support this: https://docs.aws.amazon.com/AmazonRDS/latest/PostgreSQLReleaseNotes/postgresql-extensions.html
CREATE EXTENSION btree_gist;

-- TODO: consider to create a role for the application
