### Running Tests

datajoint_core uses docker to run many compatible database systems for integration testing. You'll need to [install docker](https://docs.docker.com/engine/) to run the full suite. You can validate your docker installation with:

    $ docker run hello-world

Build all docker services first using:

    $ docker-compose -f .\docker-compose-test.yml build

Run all tests against all supported databases in docker using:

    $ docker-compose -f .\docker-compose-test.yml up
