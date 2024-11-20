#!/bin/sh

openssl req -x509 \
  -newkey rsa:4096 -keyout client_private_key.pem \
  -out client_certificate.pem -sha256 -days 3650 -nodes \
  -subj "/C=DE/O=SampleOrganization/CN=Open62541Client@localhost"
