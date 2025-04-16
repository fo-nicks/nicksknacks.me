#!/bin/bash

# Full credit to this StackOverflow answer for providing most of the legwork here:
#   https://stackoverflow.com/a/60516812

# For Windows: 
# 1. Import the $NAME.pfx into the Trusted Certificate Authorities of Windows by opening (double-click) the $NAME.pfx file, 
# 2. Select "Local Machine" and Next, Next again, enter the password and then Next, 
# 3. Select "Place all certificates int he following store:" and click on Browse 
# 4. Choose "Trusted Root Certification Authorities" and Next, and then Finish.

# For Mac
# 1. Import the CA cert at "File > Import file", 
#    then also find it in the list, right click it, expand "> Trust", and select "Always"
# 2. Add extendedKeyUsage=serverAuth,clientAuth below basicConstraints=CA:FALSE,
#    and make sure you set the "CommonName" to the same as $NAME when it asks for setup.

mkdir -p certs

# Become a Certificate Authority
# Generate private key
openssl genrsa -des3 -out certs/local-ca.key 2048
# Generate root certificate
openssl req -x509 -new -nodes -key certs/local-ca.key -sha256 -days 825 -out certs/local-ca.pem

# Create CA-signed certs
NAME=localhost # Use your own domain name
# Generate a private key
openssl genrsa -out certs/$NAME.key 2048
# Create a certificate-signing request
openssl req -new -key certs/$NAME.key -out certs/$NAME.csr
# Create a config file for the extensions
>certs/$NAME.ext cat <<-EOF
authorityKeyIdentifier=keyid,issuer
basicConstraints=CA:FALSE
keyUsage = digitalSignature, nonRepudiation, keyEncipherment, dataEncipherment
subjectAltName = @alt_names
[alt_names]
DNS.1 = $NAME # Be sure to include the domain name here because Common Name is not so commonly honoured by itself
DNS.2 = www.$NAME # Optionally, add additional domains (I've added a subdomain here)
# IP.1 = 0.0.0.0 # Optionally, add an IP address (if the connection which you have planned requires it)
EOF
# Create the signed certificate
openssl x509 -req -in certs/$NAME.csr -CA certs/local-ca.pem -CAkey certs/local-ca.key -CAcreateserial \
-out certs/$NAME.crt -days 825 -sha256 -extfile certs/$NAME.ext
openssl pkcs12 -export -out certs/local-ca.pfx -inkey certs/local-ca.key -in certs/local-ca.pem
