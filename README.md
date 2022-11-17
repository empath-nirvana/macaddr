This cli tool will query the api at https://macaddress.io and return the name of the vendor of the device.

You have to sign up for an api key and set it as an environment variable called MAC_ADDR_API 

To build it, you'll have to install the rust tool chain and run `cargo build --release`

There are prebuilt releases for it for windows, linux and mac that you can download on the github releases page.  If you want to download the mac version, you'll have to download it using curl, instead of the browser.

You can also just build the docker image with `docker build . -t <image name>`



```
export MAC_API_KEY=<api key>
macaddr <MAC_ADDRESS>
```

To run the docker image, run:

```
export MAC_API_KEY=<api key>
docker run -e MAC_API_KEY=${MAC_API_KEY} empath/macaddr <MAC_ADDRESS>
```
