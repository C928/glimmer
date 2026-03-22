## Glimmer - Experimental project

### Usage

```sh
# start the redis instance
docker run --name redis-instance-name -p 6379:6379 -d redis

# start the server on port 8080
# when fetching the root page, you should see an empty map (as no locations are present in the redis SET)
cargo run

# this will populate the redis instance with fake user locations data
cargo test --test redis

# connect to redis instance in the docker container
docker exec -it redis-instance-name redis-cli

# "locations" SET should be present
keys *

# once the data is present click the Refresh Map button on the webpage and the map will get populated with fake data.
# to fetch locations from the actix-web server, HTTP streaming is used to avoid filling up server memory (loading all
# locations from redis then transmitting to client)
```

### Demo
[demo.webm](https://github.com/user-attachments/assets/cd1f8674-cb23-459d-bb5c-7b532a4fc572)

