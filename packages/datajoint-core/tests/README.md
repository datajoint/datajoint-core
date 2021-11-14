### Running Tests

datajoint_core uses docker to run many compatible database systems for integration testing. You'll need to [install docker](https://docs.docker.com/engine/) to run the full suite. You can validate your docker installation with:

    $ docker run hello-world

Start the databases with `docker-compose` before running tests:

    $ docker-compose up

Run all tests against all supported databases using:

    $ ./run_tests.py
