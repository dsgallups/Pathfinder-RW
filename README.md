# Information
Run on `Ubuntu 20.04`

# For installing `diesel_cli`
```
sudo apt install sqlite3
sudo apt install mysql-server
sudo apt-get install libpq5=12.12-0ubuntu0.20.04.1 && sudo apt-get install libpq-dev

sudo apt-get install libsqlite3-dev
sudo apt install mysql-client
sudo apt-get install default-libmysqlclient-dev
```

# Installing and configuring PostGRES
```
sudo apt install wget ca-certificates
wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo apt-key add -
sudo sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt/ $(lsb_release -cs)-pgdg main" >> /etc/apt/sources.list.d/pgdg.list'
sudo apt update
sudo apt install postgresql postgresql-contrib
```

## In PostGRES GUI via `sudo -u postgres psql`
```
ALTER USER postgres PASSWORD 'postgres';
CREATE DATABASE diesel_demo
```