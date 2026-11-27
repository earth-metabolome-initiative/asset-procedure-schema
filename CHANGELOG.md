# Changelog

All notable changes to this project will be documented in this file.

## Unreleased

* Introduced the core tables for assets and procedures previously used internally in EMI projects.

* Rebuilt the core schema on the `resources` of `connetto-schema`, with row level security on every asset, template and procedure table.
* Procedure templates are versioned blueprints that freeze on publish and declare slots, composite steps, slot bindings with child and parent roles, repeated steps and typed parameters.
* Procedures run a published template, record the assets filling each slot, parameter values and step runs, and check that an asset exists for the whole time a procedure uses it.
* A step of a run can be skipped, and skipping a required step records the deviation with its reason. Every procedure names the root procedure of its run.
* Shared assets and shared procedures are ported to the slot model.
* A slot can name the slot of its template whose assets it is placed into or retrieved from. Container occupancy and capacity follow these links for every template, and a compatibility rule must admit each linked pair.
* Every asset keeps a ledger of the runs that produce, consume, place or retrieve it, readable by its readers, and container occupancy records each placement with the compatibility rule it counts against. Lifetime, whereabouts and capacity checks read these ledgers, so they hold across projects without revealing other projects' runs.
* A run's performer can only be the user who names them, and an in-progress or completed run that uses an asset cannot be deleted.
* A compatibility rule or volumetric container model is frozen once a template or run relies on it.
* Whoever reads a model reads its ancestors and their rules, and whoever reads a template reads the child templates its steps run.
* The compatibility rules of an organization neither admit a holder slot nor count a placement of a caller it blocks.
* APS rows follow their `resources` project parent and archive state through cascading keys.