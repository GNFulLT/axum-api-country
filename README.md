Example config toml :

```
[env]
POSTGRE_URL="postgres://{username}:{pwd}@{host}/{db_name}"
POOL_SIZE="10"
# To prevent automatically sql running after first setup
RESET_DB="false"
```
