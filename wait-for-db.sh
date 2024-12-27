
host="$1"
shift
cmd="$@"

until mysqladmin ping -h"$host" --silent; do
  echo "Waiting for database at $host..."
  sleep 2
done

echo "Database is ready. Executing command..."
diesel migration run
exec $cmd