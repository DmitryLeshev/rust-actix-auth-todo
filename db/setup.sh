PGPASSWORD=password-dima psql -h 127.0.0.1 -p 5432 -d application -f structure.sql -U dima
PGPASSWORD=password-dima psql -h 127.0.0.1 -p 5432 -d application -f data.sql -U dima