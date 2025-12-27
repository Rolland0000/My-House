echo ===================
echo    Start The App
echo ===================

set -x
set -eo

#Check dependancies
#check if psql is install
if ! [ -x "$(command -v psql)"]; then
    echo >&2 "❌ Error: psql is not installed."
    exit 1
fi
#check if sqlw is installed or not
if ! [-x "$(command -sqlx)"]; then
  echo >&2 "❌ Error: sqlx is not installed."
  echo >&2 " cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres"
  exit 1
fi

#check if custom informations of the database have been set, otherwise set default values
DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"

# Launch postgres using Docker with maximum number of connection pool set 1000
docker run \
-e POSTGRES_USER=${DB_USER} \
-e POSTGRES_PASSWORD=${DB_PASSWORD} \
-e POSTGRES_DB=${DB_NAME} \
-p "${DB_PORT}":5432 \
-d postgres \
postgres -N 1000

