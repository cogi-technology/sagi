#!/bin/bash
set -ex

NAME=service

function create() {
    if [ $# -ne 3 ] && [ $# -ne 4 ]; then
        echo "usage: create_db <database> <user> <password> [<file.sql>]"
        exit
    fi
    db=$1
    user=$2
    pswd=$3
    port=$4
    echo "Synchronizer database creation"
    echo "Installing PostgreSQL..."
    docker run -d --name $NAME -p $port:5432 -e POSTGRES_PASSWORD=$pswd -e POSTGRES_USER=$user -e POSTGRES_DB=$db postgres
    sleep 2
    echo "Creating table playing..."
    if [ $# == 5 ]; then
        docker exec -ti $NAME psql -U $user -d $db -f $5
    fi

    echo "Done."
    echo "Example of connection string to use in the config.json file:"
    echo "  \"databaseURL\": \"postgresql://$user:$pswd@127.0.0.1:${PORT}/$db\""
    echo
}

function delete() {
    if [ $# -ne 2 ]
        then
            echo "usage: delete_db <database> <user>"
            exit
    fi
    echo "StateDB database deletion"
    echo "Deleting database $1..."
    docker rm -f $NAME
    echo "Done."
    echo
}

action=$1
shift
$action $@