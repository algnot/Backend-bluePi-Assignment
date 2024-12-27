#!/bin/bash

host="$1"
shift
cmd="$@"

echo "Delay for database $host..."
sleep 10

# Check if the migrations directory exists
if [ ! -d "./migrations" ]; then
  echo "Error: Migrations directory not found!"
  exit 1
fi

# Run Diesel migrations
diesel migration run --migration-dir ./migrations

# Execute the specified command
exec $cmd