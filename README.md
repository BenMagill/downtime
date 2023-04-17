# Downtime
Simple rust app for checking the downtime of sites

## TODO
- Finish basic version
    - Get uris from db
    - Store metrics in db
    - Provide UI / CLI for viewing and adding
- Report downtime to webhook endpoint

## Setup pg container
docker run --name postgres-db -e POSTGRES_PASSWORD=docker -p 5432:5432 -d postgres
