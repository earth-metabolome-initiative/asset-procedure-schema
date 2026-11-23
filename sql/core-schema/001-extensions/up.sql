-- PostGIS provides the `GEOGRAPHY` type that geolocation procedures record.
-- The version is the one whose function signatures the grant below names.
CREATE EXTENSION IF NOT EXISTS postgis VERSION '3.7.0';
GRANT EXECUTE ON FUNCTION geography(geography, integer, boolean) TO app;
