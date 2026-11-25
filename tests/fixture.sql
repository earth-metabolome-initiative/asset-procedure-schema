-- One organization with one project, founded by the first of two users, and helpers that author
-- templates and run procedures in that project. Each behaviour file includes it inside its own
-- transaction and continues as the founder under the `app` role.
SET ROLE identity_writer;
INSERT INTO users (id) VALUES ('10000000-0000-7000-8000-000000000001'), ('10000000-0000-7000-8000-000000000002');
RESET ROLE;

CREATE FUNCTION pg_temp.act(principal TEXT) RETURNS VOID LANGUAGE sql AS $$
  SELECT set_config('app.user_id', coalesce(principal, ''), false), set_config('app.installation_id', '', false),
         set_config('app.link_ids', '', false), set_config('app.original_actor_id', '', false),
         set_config('app.verified_email', '', false)
$$;
CREATE FUNCTION pg_temp.create_ownable(ownable UUID) RETURNS VOID LANGUAGE sql AS $$
  INSERT INTO public.ownables (id, organization_id, project_id, created_by_id)
    VALUES (ownable, '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', authz.caller_id());
  INSERT INTO public.resources (id, kind, organization_id) VALUES (ownable, 'ownable', '20000000-0000-7000-8000-000000000001');
$$;
-- The caller invites the member, who accepts, as an organization admin and the invitee do.
CREATE FUNCTION pg_temp.add_member(organization UUID, member UUID) RETURNS VOID LANGUAGE plpgsql AS $$
DECLARE
  inviter TEXT := current_setting('app.user_id');
  email TEXT := current_setting('app.verified_email');
  invitation UUID := gen_random_uuid();
BEGIN
  INSERT INTO public.invitations (id, organization_id, email, expires_at, invited_by_id)
    VALUES (invitation, organization, member || '@example.com', now() + interval '1 day', inviter::uuid);
  PERFORM set_config('app.user_id', member::text, false), set_config('app.verified_email', member || '@example.com', false);
  INSERT INTO public.memberships (organization_id, user_id, state, invitation_id) VALUES (organization, member, 'active', invitation);
  PERFORM set_config('app.user_id', inviter, false), set_config('app.verified_email', email, false);
END
$$;
-- A model with its complete ancestor rows, held by `model_table` as its most concrete table.
CREATE FUNCTION pg_temp.new_model(model UUID, model_name TEXT, parent UUID, model_table TEXT DEFAULT 'asset_models')
RETURNS VOID LANGUAGE sql AS $$
  SELECT pg_temp.create_ownable(model);
  INSERT INTO public.asset_models (id, organization_id, project_id, table_name_id, name, parent_model_id)
    VALUES (model, '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', model_table, model_name, parent);
  INSERT INTO public.asset_model_ancestors (descendant_model_id, ancestor_model_id, organization_id, parent_model_id)
    VALUES (model, model, '20000000-0000-7000-8000-000000000001', NULL);
  INSERT INTO public.asset_model_ancestors (descendant_model_id, ancestor_model_id, organization_id, parent_model_id)
    SELECT model, ancestor.ancestor_model_id, '20000000-0000-7000-8000-000000000001', parent
    FROM public.asset_model_ancestors ancestor WHERE ancestor.descendant_model_id = parent;
$$;
CREATE FUNCTION pg_temp.new_asset(asset UUID, asset_name TEXT, model UUID, asset_table TEXT DEFAULT 'assets')
RETURNS VOID LANGUAGE sql AS $$
  SELECT pg_temp.create_ownable(asset);
  INSERT INTO public.assets (id, organization_id, project_id, table_name_id, model_id, name)
    VALUES (asset, '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', asset_table, model, asset_name);
$$;
CREATE FUNCTION pg_temp.new_template(template UUID, template_name TEXT, template_table TEXT DEFAULT 'procedure_templates')
RETURNS VOID LANGUAGE sql AS $$
  SELECT pg_temp.create_ownable(template);
  INSERT INTO public.procedure_templates (id, organization_id, project_id, table_name_id, name)
    VALUES (template, '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', template_table, template_name);
$$;
CREATE FUNCTION pg_temp.new_slot(slot UUID, template UUID, slot_name TEXT, model UUID, role TEXT, minimum INTEGER, maximum INTEGER)
RETURNS VOID LANGUAGE sql AS $$
  INSERT INTO public.procedure_template_slots
    (id, organization_id, procedure_template_id, name, asset_model_id, role_id, minimum_count, maximum_count)
    VALUES (slot, '20000000-0000-7000-8000-000000000001', template, slot_name, model, role, minimum, maximum);
$$;
CREATE FUNCTION pg_temp.new_step(step UUID, parent UUID, child UUID, step_name TEXT, step_position INTEGER) RETURNS VOID LANGUAGE sql AS $$
  INSERT INTO public.procedure_template_steps (id, organization_id, parent_template_id, child_template_id, name, position)
    VALUES (step, '20000000-0000-7000-8000-000000000001', parent, child, step_name, step_position);
$$;
CREATE FUNCTION pg_temp.new_edge(predecessor UUID, successor UUID) RETURNS VOID LANGUAGE sql AS $$
  INSERT INTO public.procedure_template_step_edges
    (predecessor_step_id, successor_step_id, parent_template_id, organization_id, predecessor_position, successor_position)
    SELECT before.id, after.id, before.parent_template_id, before.organization_id, before.position, after.position
    FROM public.procedure_template_steps before, public.procedure_template_steps after
    WHERE before.id = predecessor AND after.id = successor;
$$;
CREATE FUNCTION pg_temp.new_binding(step UUID, child_slot UUID, parent_slot UUID) RETURNS VOID LANGUAGE sql AS $$
  INSERT INTO public.procedure_template_slot_bindings
    (step_id, child_slot_id, parent_template_id, child_template_id, parent_slot_id, child_asset_model_id, parent_asset_model_id,
      child_role_id, parent_role_id, organization_id)
    SELECT step, child_slot, parent.procedure_template_id, child.procedure_template_id, parent_slot, child.asset_model_id, parent.asset_model_id,
      child.role_id, parent.role_id, parent.organization_id
    FROM public.procedure_template_slots child, public.procedure_template_slots parent
    WHERE child.id = child_slot AND parent.id = parent_slot;
$$;
CREATE FUNCTION pg_temp.new_procedure(procedure UUID, template UUID, status TEXT, parent UUID, step UUID, repeat UUID DEFAULT NULL,
    procedure_table TEXT DEFAULT 'procedures')
RETURNS VOID LANGUAGE sql AS $$
  SELECT pg_temp.create_ownable(procedure);
  INSERT INTO public.procedures (id, organization_id, project_id, table_name_id, procedure_template_id, status_id,
      parent_procedure_id, parent_template_id, step_id, root_procedure_id, repeat_procedure_asset_id, started_at, performed_by_id)
    SELECT procedure, '20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', procedure_table, template, status,
      parent, (SELECT parent_procedure.procedure_template_id FROM public.procedures parent_procedure WHERE parent_procedure.id = parent),
      step, coalesce((SELECT parent_procedure.root_procedure_id FROM public.procedures parent_procedure WHERE parent_procedure.id = parent), procedure),
      repeat, CASE WHEN status = 'planned' THEN NULL ELSE now() END, authz.caller_id();
$$;
-- Binds an asset, or only a model when `asset` is NULL, inheriting the
-- parent's row when the procedure's step binds the slot.
CREATE FUNCTION pg_temp.bind_model(binding UUID, procedure UUID, slot UUID, asset UUID, model UUID) RETURNS VOID LANGUAGE sql AS $$
  INSERT INTO public.procedure_assets (id, organization_id, procedure_id, procedure_template_id, status_id, slot_id,
      slot_asset_model_id, slot_role_id, asset_model_id, asset_id, parent_procedure_asset_id, parent_procedure_id)
    SELECT binding, executed.organization_id, procedure, executed.procedure_template_id, executed.status_id, slot,
      template_slot.asset_model_id, template_slot.role_id, model, asset, inherited.id,
      CASE WHEN inherited.id IS NOT NULL THEN executed.parent_procedure_id END
    FROM public.procedures executed
    JOIN public.procedure_template_slots template_slot ON template_slot.id = slot
    LEFT JOIN LATERAL (
      SELECT inherited.id FROM public.procedure_template_slot_bindings step_binding
        JOIN public.procedure_template_steps step ON step.id = step_binding.step_id
        JOIN public.procedure_assets inherited
          ON inherited.procedure_id = executed.parent_procedure_id AND inherited.slot_id = step_binding.parent_slot_id
      WHERE step_binding.step_id = executed.step_id AND step_binding.child_slot_id = slot
        AND inherited.asset_id IS NOT DISTINCT FROM asset
        AND (step.repeat_slot_id IS DISTINCT FROM step_binding.parent_slot_id OR inherited.id = executed.repeat_procedure_asset_id)
      LIMIT 1) inherited ON true
    WHERE executed.id = procedure;
$$;
CREATE FUNCTION pg_temp.bind(binding UUID, procedure UUID, slot UUID, asset UUID) RETURNS VOID LANGUAGE sql AS $$
  SELECT pg_temp.bind_model(binding, procedure, slot, asset, (SELECT bound.model_id FROM public.assets bound WHERE bound.id = asset));
$$;
CREATE FUNCTION pg_temp.record_number(procedure UUID, parameter UUID, recorded DOUBLE PRECISION) RETURNS VOID LANGUAGE sql AS $$
  INSERT INTO public.procedure_parameter_values
    (procedure_id, parameter_id, procedure_template_id, status_id, value_kind_id, organization_id, number_value)
    SELECT procedure, parameter, executed.procedure_template_id, executed.status_id, 'number', executed.organization_id, recorded
    FROM public.procedures executed WHERE executed.id = procedure;
$$;
GRANT EXECUTE ON FUNCTION pg_temp.create_ownable(UUID), pg_temp.add_member(UUID, UUID), pg_temp.new_model(UUID, TEXT, UUID, TEXT),
  pg_temp.new_asset(UUID, TEXT, UUID, TEXT),
  pg_temp.new_template(UUID, TEXT, TEXT), pg_temp.new_slot(UUID, UUID, TEXT, UUID, TEXT, INTEGER, INTEGER),
  pg_temp.new_step(UUID, UUID, UUID, TEXT, INTEGER), pg_temp.new_edge(UUID, UUID), pg_temp.new_binding(UUID, UUID, UUID),
  pg_temp.new_procedure(UUID, UUID, TEXT, UUID, UUID, UUID, TEXT), pg_temp.bind_model(UUID, UUID, UUID, UUID, UUID),
  pg_temp.bind(UUID, UUID, UUID, UUID), pg_temp.record_number(UUID, UUID, DOUBLE PRECISION) TO app;

SELECT pg_temp.act('10000000-0000-7000-8000-000000000001');
SET ROLE app;
INSERT INTO organizations (id, created_by_id) VALUES ('20000000-0000-7000-8000-000000000001', authz.caller_id());
INSERT INTO resources (id, kind, organization_id)
  VALUES ('20000000-0000-7000-8000-000000000001', 'organization', '20000000-0000-7000-8000-000000000001');
INSERT INTO memberships (organization_id, user_id, state) VALUES ('20000000-0000-7000-8000-000000000001', authz.caller_id(), 'active');
INSERT INTO grants (organization_id, resource_id, grantee_id, grantee_kind, role)
  VALUES ('20000000-0000-7000-8000-000000000001', '20000000-0000-7000-8000-000000000001', authz.caller_id(), 'user', 'admin');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;
INSERT INTO resource_parents (organization_id, id, resource_kind, parent_id, parent_kind)
  VALUES ('20000000-0000-7000-8000-000000000001', '50000000-0000-7000-8000-000000000001', 'project',
    '20000000-0000-7000-8000-000000000001', 'organization');
INSERT INTO resources (id, kind, organization_id)
  VALUES ('50000000-0000-7000-8000-000000000001', 'project', '20000000-0000-7000-8000-000000000001');
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS ALL DEFERRED;

