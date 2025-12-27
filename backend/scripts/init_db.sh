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
DB_USER=${POSTGRES_USER:=Rolland_0}
DB_PASSWORD="${POSTGRES_PASSWORD:=m@vie_321}"
DB_NAME="${POSTGRES_DB:=my_house}"
DB_PORT="${POSTGRES_PORT:=5433}"

# Launch postgres using Docker with maximum number of connection pool set 1000
#skip this step is there is an already runing instance of our docker datbase
if [[ -z "${SKIP_DOCKER}" ]]
then
  echo "🚀 Starting PostgreSQL container with Docker..."
  docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000
fi

# Keep pinging Postgres until it's ready to accept commands
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
>&2 echo "Postgres is still unavailable - sleeping"
sleep 1
done
>&2 echo "Postgres is up and running on port ${DB_PORT}!"
export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create