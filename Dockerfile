# We choose Ubuntu 20.04 LTS (Focal Fossa) as our base. 
# It's an old, stable distribution, ensuring maximum glibc compatibility 
# with modern Linux systems.
FROM ubuntu:20.04

# Avoid prompts during package installation, especially for 'tzdata'.
ENV DEBIAN_FRONTEND=noninteractive

# Install essential build tools and dependencies required by the Rust project (e.g., GTK).
RUN apt update && apt install -y \
    wget \
    build-essential \
    curl \
    git \
    libgtk-3-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Install Rust toolchain via rustup.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
# Add Cargo binaries to the PATH.
ENV PATH="/root/.cargo/bin:${PATH}"

# Set the working directory inside the container.
WORKDIR /app

# Copy the Rust source code from the host to the container. 
# Note: Ensure a .dockerignore file excludes 'target/', '.git/', etc.
COPY . .

# Compile the Rust application in release mode. 
# The binary will be named 'skyscrapers' (lowercase) based on Cargo.toml.
RUN cargo build --release

#--------------------------------------------------------------------------
# AppImage Creation Phase
#--------------------------------------------------------------------------

# 1. Download only the core linuxdeploy tool.
RUN wget -c -O linuxdeploy https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage \
    && chmod +x linuxdeploy

# 2. Extract linuxdeploy to bypass FUSE.
RUN ./linuxdeploy --appimage-extract

# prepare AppDir structure
ENV APP=skyscrapers
ENV APP_DIR=${APP}.AppDir
RUN mkdir -p ${APP_DIR}/usr/bin

# Copy the compiled binary (lowercase 'skyscrapers')
RUN cp target/release/skyscrapers ${APP_DIR}/usr/bin/skyscrapers
RUN chmod +x ${APP_DIR}/usr/bin/skyscrapers

# Create a robust AppRun script to ensure the executable is found
RUN echo '#!/bin/sh' > ${APP_DIR}/AppRun \
    && echo 'exec "$APPDIR"/usr/bin/skyscrapers "$@"' >> ${APP_DIR}/AppRun \
    && chmod +x ${APP_DIR}/AppRun

# Create the desktop file
RUN cat << EOF > ${APP_DIR}/${APP}.desktop
[Desktop Entry]
Version=1.0
Type=Application
Name=Skyscrapers
Exec=skyscrapers
Icon=skyscrapers_icon
Categories=Game;
EOF

# Copy the icon file
COPY skyscrapers.png ${APP_DIR}/skyscrapers_icon.png

# 3. Final step: Execute linuxdeploy *without* the external plugin call.
# linuxdeploy will automatically bundle libraries found via LDD, 
# which is sufficient for many GTK applications compiled on an old base.
RUN squashfs-root/AppRun --appdir ${APP_DIR} --output appimage 
# No --plugin gtk argument needed!