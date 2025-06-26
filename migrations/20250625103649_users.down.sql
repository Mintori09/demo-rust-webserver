-- Down migration
DROP INDEX IF EXISTS user_email_idx;
DROP TABLE IF EXISTS "users";
DROP TYPE IF EXISTS user_role;
DROP EXTENSION IF EXISTS "uuid-ossp";
