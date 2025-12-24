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
