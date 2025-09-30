# Skyscrapers

Skyscrapers logic game.

You must find the height of all buildings in the grid, given the clues around it.

## Building AppImage (Linux users)

0. Install Docker CLI and go in the project root folder with your terminal
1. build the docker image : `docker build -t rust-appimage-builder .`
2. extract program from docker image

```bash
APPIMAGE_NAME=Skyscrapers-x86_64.AppImage
docker create --name temp_extractor rust-appimage-builder
docker cp temp_extractor:/app/${APPIMAGE_NAME} .
docker rm temp_extractor
```

## Credits

- Icon has been downloaded from [Svg Repo](https://www.svgrepo.com/svg/42651/skyscrapers)
