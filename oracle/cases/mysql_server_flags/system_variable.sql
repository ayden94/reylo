select @@max_allowed_packet as packet, @@session.sql_mode as mode, @@global.max_connections as connections, @@autocommit as autocommit
