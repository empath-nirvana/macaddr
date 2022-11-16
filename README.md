This cli tool will query the api at https://macaddress.io and return the name of the vendor name of the device.

You have to sign up for an api key and set it as an environment variable called MAC_ADDR_API 

To build it, you'll have to install the rust tool chain and run `cargo build --release`

Instead of installing rust, you can just build the docker image with `docker build . -t empath/macaddr`  (or whatever you want to call it)

To run the binary, run 

```
export MAC_API_KEY=<api key>
macaddr <MAC_ADDRESS>
```

To run the docker image, run:

```
export MAC_API_KEY=<api key>
docker run -e MAC_API_KEY=${MAC_API_KEY} empath/macaddr <MAC_ADDRESS>
```