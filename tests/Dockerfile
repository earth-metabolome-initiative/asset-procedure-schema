ARG PG_MAJOR=18
FROM postgres:${PG_MAJOR}
RUN apt-get update \
  && apt-get install -y --no-install-recommends \
    "postgresql-${PG_MAJOR}-pgtap" \
    "postgresql-${PG_MAJOR}-plpgsql-check" \
    "postgresql-${PG_MAJOR}-postgis-3" \
    libtap-parser-sourcehandler-pgtap-perl \
  && rm -rf /var/lib/apt/lists/*
