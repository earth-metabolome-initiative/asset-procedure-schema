\set ON_ERROR_STOP 1
SET search_path = public, test_tools;
BEGIN;
\ir ../fixture.sql
SELECT plan(12);

-- An INSERT grant alone does not open a catalog table, whose policies only let `app` read it.
RESET ROLE;
GRANT INSERT ON table_names, slot_roles, slot_role_pairs, parameter_value_kinds, procedure_template_states, procedure_statuses TO app;
SET ROLE app;

SELECT isnt_empty($query$SELECT id FROM table_names$query$, 'table_names is readable');
SELECT throws_ok($statement$INSERT INTO table_names (id) VALUES ('forbidden')$statement$, '42501',
  'new row violates row-level security policy for table "table_names"', 'table_names refuses app inserts');
SELECT isnt_empty($query$SELECT id FROM slot_roles$query$, 'slot_roles is readable');
SELECT throws_ok($statement$INSERT INTO slot_roles (id) VALUES ('forbidden')$statement$, '42501',
  'new row violates row-level security policy for table "slot_roles"', 'slot_roles refuses app inserts');
SELECT isnt_empty($query$SELECT child_role_id FROM slot_role_pairs$query$, 'slot_role_pairs is readable');
SELECT throws_ok($statement$INSERT INTO slot_role_pairs (child_role_id, parent_role_id) VALUES ('instrument', 'input')$statement$, '42501',
  'new row violates row-level security policy for table "slot_role_pairs"', 'slot_role_pairs refuses app inserts');
SELECT isnt_empty($query$SELECT id FROM parameter_value_kinds$query$, 'parameter_value_kinds is readable');
SELECT throws_ok($statement$INSERT INTO parameter_value_kinds (id) VALUES ('forbidden')$statement$, '42501',
  'new row violates row-level security policy for table "parameter_value_kinds"', 'parameter_value_kinds refuses app inserts');
SELECT isnt_empty($query$SELECT id FROM procedure_template_states$query$, 'procedure_template_states is readable');
SELECT throws_ok($statement$INSERT INTO procedure_template_states (id) VALUES ('forbidden')$statement$, '42501',
  'new row violates row-level security policy for table "procedure_template_states"', 'procedure_template_states refuses app inserts');
SELECT isnt_empty($query$SELECT id FROM procedure_statuses$query$, 'procedure_statuses is readable');
SELECT throws_ok($statement$INSERT INTO procedure_statuses (id) VALUES ('forbidden')$statement$, '42501',
  'new row violates row-level security policy for table "procedure_statuses"', 'procedure_statuses refuses app inserts');

RESET ROLE;
SELECT * FROM finish();
ROLLBACK;
