[Paper Wars]: https://github.com/foxfriends/paper-wars
[Postgres]: https://www.postgresql.org/

# Paper Wars Database

Database schema and models for [Paper Wars][].

## Setup

1.  Install [Postgres][].
2.  Install Rust (see the [main README][Paper Wars] for instructions)
3.  Install Diesel

    ```sh
    cargo install diesel_cli --no-default-features --features 'postgres'
    ```
4.  Set everything up. You will be asked for some passwords, just pick whatever because this
    is a development password:

    ```sh
    createuser paper-wars
    createuser paper-wars-admin -lPes -g paper-wars
    createuser paper-wars-server -lPe -g paper-wars
    ```
5.  Copy the `.env.sample` file to `.env` and set the credentials to the ones created in the 
    previous step.

    ```sh
    DATABASE_URL=postgres://paper-wars-admin:<password>@localhost/paper-wars
    ```
6.  Everything should now be working:

    ```sh
    diesel database setup
    ```
