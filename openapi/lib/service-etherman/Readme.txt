## private_key
openssl genpkey -algorithm RSA -out private_key.pem
## public_key
openssl rsa -pubout -in private_key.pem -out public_key.pem