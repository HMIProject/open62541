#!/bin/sh

openssl req -x509 \
  -newkey rsa:4096 -keyout server_private_key.pem \
  -out server_certificate.pem -sha256 -days 3650 -nodes \
  -subj "/C=DE/O=SampleOrganization/CN=Open62541Server@localhost" \
  -addext "keyUsage=digitalSignature,nonRepudiation" \
  -addext "subjectAltName=DNS:localhost,URI:urn:open62541.server.application"
