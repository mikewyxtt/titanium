## Initializing the source tree
`git submodule update --init --recursive`

## Building and running the Docker Image
`docker build -t titanium .`

`docker run -it --rm -v $(pwd):/home/titanium/titanium-src titanium`
