# HotReloader

Re-execute command when a file changes

## Installation

### From Github releases page

Go to [Release page](https://github.com/danny270793/hotreloader/releases) then download the binary which fits your environment

### From terminal

Get the last versión available on github

```bash
LAST_VERSION=$(curl https://api.github.com/repos/danny270793/hotreloader/releases/latest | grep tag_name | cut -d '"' -f 4)
```

Download the last version directly to the binaries folder

For Linux (linux):

```bash
curl -L https://github.com/danny270793/hotreloader/releases/download/${LAST_VERSION}/hotreloader -o ./hotreloader
```

Then copy the binary to the binaries folder

```bash
sudo cp ./hotreloader /usr/local/bin/hotreloader
```

Make it executable the binary

```bash
sudo chmod +x /usr/local/bin/hotreloader
```

```bash
hotreloader version
```

## Ussage

Run the binary and pass the binary and the file to re-execute every tima a file changes on the parent folder of the file

```bash
hotreloader -- node ./example/index.js
```

## Follow me

- [Youtube](https://www.youtube.com/channel/UC5MAQWU2s2VESTXaUo-ysgg)
- [Github](https://www.github.com/danny270793/)
- [LinkedIn](https://www.linkedin.com/in/danny270793)

## LICENSE

Licensed under the [MIT](license.md) License

## Version

RustHotReloader version 1.0.0

Last update 10/07/2024