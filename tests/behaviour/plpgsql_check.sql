\set ON_ERROR_STOP 1
SET search_path = public, test_tools;
BEGIN;
SELECT plan(1);

SELECT is_empty($query$
  WITH schema_functions AS (
    SELECT function.oid, function.prorettype
    FROM pg_proc function
    JOIN pg_language language ON language.oid = function.prolang AND language.lanname = 'plpgsql'
    JOIN pg_namespace namespace ON namespace.oid = function.pronamespace
    WHERE namespace.nspname NOT IN ('pg_catalog', 'information_schema', 'test_tools')
      AND NOT EXISTS (
        SELECT 1 FROM pg_depend dependency
        WHERE dependency.classid = 'pg_proc'::regclass AND dependency.objid = function.oid AND dependency.deptype = 'e'
      )
  ), targets AS (
    SELECT schema_functions.oid AS function_id, 0::oid AS relation_id
    FROM schema_functions
    WHERE schema_functions.prorettype <> 'trigger'::regtype
    UNION ALL
    SELECT trigger.tgfoid, trigger.tgrelid
    FROM pg_trigger trigger
    JOIN schema_functions ON schema_functions.oid = trigger.tgfoid
    WHERE NOT trigger.tgisinternal
  )
  SELECT targets.function_id::regprocedure, targets.relation_id::regclass, finding.lineno, finding.level, finding.message
  FROM targets,
    LATERAL plpgsql_check_function_tb(
      targets.function_id,
      targets.relation_id,
      performance_warnings => true,
      extra_warnings => true,
      security_warnings => true
    ) finding
$query$, 'plpgsql_check reports nothing on any PL/pgSQL function or trigger the schema defines');

SELECT * FROM finish();
ROLLBACK;
