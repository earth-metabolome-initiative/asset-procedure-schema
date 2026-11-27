-- Owns the helpers that lock the template, procedure and asset rows a guard's check reads.
CREATE ROLE aps_writer NOLOGIN BYPASSRLS;
