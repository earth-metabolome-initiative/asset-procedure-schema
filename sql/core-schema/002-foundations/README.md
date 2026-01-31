# 002-Foundations

This directory contains the core foundational schema definitions for the Asset Procedure system. It establishes the base types for identities, assets, synchronization, and security.

This module is designed to be executed sequentially, building the schema graph from independent entities up to complex authorized structures.

## Modules

### 001-Entities
Establishes the root tracking mechanisms for the system.
- **table_names**: A registry of all tables in the system for dynamic type identification.
- **entities**: An abstract base table intended for universal ID generation (UUIDv7) and audit timestamps (`created_at`, `edited_at`).

### 002-Sync
Provides mechanisms for distributed data consistency (Offline-First support).
- **tombstones**: Records deleted entity IDs and their table types. This allows offline clients to query "What has been deleted since I last synced?" and reconcile their local database.

### 003-Owners
Defines the **Base Identity** layer.
- **owners**: Abstract base table for any entity that can own resources or contain members (e.g., Users, Teams). Inherits IDs from `entities`.
- **owner_tables**: Registry of valid owner sub-types (Dynamic Enum).
- **roles** & **owner_grants**: Infrastructure for capability-based permission grants between owners (ACL system).

### 004-Users
Defines the **Concrete Identity** for individual humans.
- **users**: Extends `owners`. Represents an authenticated human user.
- **Auth Functions**: Helper functions to retrieve collection/session context (e.g., `auth.current_user_id()`).

### 005-Ownables
Defines the **Base Asset** layer.
- **ownables**: Abstract base table for all managed resources (e.g., Assets, Procedures). Inherits IDs from `entities`.
- **ownable_tables**: Registry of valid ownable sub-types.
- Contains standard audit fields linking resources to their `owner_id`, `creator_id`, and `editor_id`.

### 006-Teams
Defines **Group Identities** and **Composite Assets**.
- **teams**: A "Diamond" entity that inherits from BOTH `owners` (Identity) and `ownables` (Resource).
    - As an **Owner**: Can be assigned permissions and own other assets.
    - As an **Ownable**: Can be created, edited, and owned by other users/teams.
- **team_members**: Junction table linking `users` to `teams`. Includes triggers to "flatten" team hierarchy for efficient recursive queries.

### 007-Security
Implements the high-level security policy engine.
- **Row Level Security (RLS)**: Policies enabled on `ownables` to restrict access processing at the database engine level.
- **Access Functions**: Logic (`get_accessible_owner_ids`) to recursively resolve all permissions a user holds via their individual account, team memberships, and explicit grants.
