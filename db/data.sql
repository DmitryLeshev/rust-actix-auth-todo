INSERT INTO "account" ("email", "first_name", "last_name", "hash_password") VALUES
    ('admin@mail.com', 'Ad', 'Min', '$argon2id$v=19$m=4096,t=192,p=16$D9Kq6q7GVx2EL5PqwjbR/BBOkJmYc2lZ4VjeFGS/rUw$lUswSUYW3vgb5m1rizgGGHDPBdQfVn7J84I9jbbhgSg'),
    ('foobar@mail.com', 'Foo', 'Bar', 'foobar'),
    ('user@mail.com', 'user', 'user', '$argon2id$v=19$m=4096,t=192,p=16$4Jt3VJK5YWOox05cijfneu9Z77gjhUHvo/plz2sNdCw$6L+iTlb5qGB/H8NUR/lNYeeObOXarifIzvjfdEglSrg'),
    ('iskandar@mail.com', 'iskandar', 'iskandar', 'iskandar');

-- email = admin@mail.com, password = admin
-- email = user@mail.com, password = user

INSERT INTO "role" ("name") VALUES
    ('admins'),
    ('users'),
    ('guests');

INSERT INTO "account_role" ("account_id", "role_id") VALUES
    (1, 1),
    (2, 2),
    (3, 2),
    (4, 2);


INSERT INTO "todolist" ("account_id", "name") VALUES
    (1, 'First TODO LIST'),
    (2, 'Second TODO LIST'),
    (3, 'Third TODO LIST'),
    (1, 'Fourth TODO LIST');


INSERT INTO "todoitem" ("todolist_id", "name", "description") VALUES
    (1, 'First ITEM', 'First ITEM description'),
    (1, 'Second ITEM', null),
    (1, 'Third ITEM', 'Third ITEM description'),
    (1, 'Fourth ITEM', 'Fourth ITEM description'),
    (2, 'First ITEM', 'First ITEM description'),
    (2, 'Second ITEM', null),
    (2, 'Third ITEM', 'Third ITEM description'),
    (2, 'Fourth ITEM', 'Fourth ITEM description'),
    (3, 'First ITEM', 'First ITEM description'),
    (3, 'Second ITEM', null),
    (3, 'Third ITEM', 'Third ITEM description'),
    (3, 'Fourth ITEM', 'Fourth ITEM description'),
    (4, 'First ITEM', 'First ITEM description'),
    (4, 'Second ITEM', null),
    (4, 'Third ITEM', 'Third ITEM description'),
    (4, 'Fourth ITEM', 'Fourth ITEM description');

-- STEP

-- INSERT INTO "step" ("name", "description") VALUES
--     ('First STEP', 'First STEP description'),
--     ('Second STEP', null),
--     ('Third STEP', 'Third STEP description'),
--     ('Fourth STEP', 'Fourth STEP description');

-- INSERT INTO "todolist_step" ("todolist_id", "step_id") VALUES
--     (1, 1),
--     (1, 2),
--     (2, 3),
--     (3, 4);

-- INSERT INTO "task" ("name", "description") VALUES
--     ('First TASK', 'First TASK description'),
--     ('Second TASK', null),
--     ('Third TASK', 'Third TASK description'),
--     ('Fourth TASK', 'Fourth TASK description');

-- INSERT INTO "step_task" ("step_id", "task_id") VALUES
--     (1, 1),
--     (1, 2),
--     (2, 3),
--     (3, 4);