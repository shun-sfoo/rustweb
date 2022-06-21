# API

## Create User

`curl -X POST -H 'Content-Type: application/json' http://127.0.0.1:8080/api/users -d @data/create_user.json`

## Login

`curl -X POST -H 'Content-Type: application/json' http://127.0.0.1:8080/api/users/login -d @data/create_user.json`

## Get Current User

`curl -H "Authorization: Bearer xxxx" http://127.0.0.1:8080/api/user`
