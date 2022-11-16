This cli tool will query the api at https://macaddress.io and return the name of the vendor name of the device.

You have to sign up for an api key and set it as an environment variable called MAC_ADDR_API (for security reasons, you should never enter secrets into the command line).

To build it, you'll have to install the rust tool chain and run `cargo build --release`

To build the docker image, just run `docker build . -t macaddr`

To run it # macaddr
