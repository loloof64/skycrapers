# We choose Ubuntu 20.04 LTS (Focal Fossa) as our base. 
# This old, stable base ensures glibc compatibility with modern Linux systems.
FROM ubuntu:20.04

# Avoid prompts during package installation (e.g., for 'tzdata').
ENV DEBIAN_FRONTEND=noninteractive

# Install essential build tools and dependencies (including GTK for the app).
RUN apt update && apt install -y \
    wget \
    build-essential \
    curl \
    git \
    libgtk-3-dev \
    libgtk-3-common \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Install Rust toolchain via rustup.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
# Add Cargo binaries to the PATH.
ENV PATH="/root/.cargo/bin:${PATH}"

# Set the working directory inside the container.
WORKDIR /app

# Copy the Rust source code from the host to the container.
COPY . .

# Compile the Rust application in release mode. 
# The binary will be named 'skyscrapers' (lowercase) based on Cargo.toml.
RUN cargo build --release

#--------------------------------------------------------------------------
# AppImage Creation Phase (Stabilized)
#--------------------------------------------------------------------------

# 1. Download only the core linuxdeploy tool.
RUN wget -c -O linuxdeploy https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage \
    && chmod +x linuxdeploy

# 2. Extract linuxdeploy to bypass FUSE requirements during the build (into ./squashfs-root).
RUN ./linuxdeploy --appimage-extract

# Prepare AppDir structure. Use lowercase for consistency with the binary name.
ENV APP=skyscrapers
ENV APP_DIR=${APP}.AppDir
RUN mkdir -p ${APP_DIR}/usr/bin

# Copy the compiled binary (ensuring correct lowercase name).
RUN cp target/release/skyscrapers ${APP_DIR}/usr/bin/skyscrapers
# Explicitly set execute permissions to avoid the common "execv error".
RUN chmod +x ${APP_DIR}/usr/bin/skyscrapers

# Create a robust AppRun script. The use of $APPDIR ensures the correct path 
# is found at runtime, resolving many execv errors.
RUN echo '#!/bin/sh' > ${APP_DIR}/AppRun \
    && echo 'exec "$APPDIR"/usr/bin/skyscrapers "$@"' >> ${APP_DIR}/AppRun \
    && chmod +x ${APP_DIR}/AppRun

# Create the desktop file (Exec must match the lowercase binary name).
RUN cat << EOF > ${APP_DIR}/${APP}.desktop
[Desktop Entry]
Version=1.0
Type=Application
Name=Skyscrapers
Exec=skyscrapers
Icon=skyscrapers_icon
Categories=Game;
EOF

# Copy the icon file (assuming 'skyscrapers.png' is in the build context).
COPY skyscrapers.png ${APP_DIR}/skyscrapers_icon.png

# Initialize the library path variable (Fixes the UndefinedVar warning)
ENV LD_LIBRARY_PATH=

# Set the library path to assist linuxdeploy in finding all necessary .so files.
ENV LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/usr/lib/x86_64-linux-gnu

# 3. Final step: Execute linuxdeploy *without* any manual copying or external plugins.
# We rely on linuxdeploy's automatic LDD (Linker Dynamic Dependency) bundling 
# to include the necessary GTK shared libraries from the Ubuntu 20.04 environment.
RUN squashfs-root/AppRun --appdir ${APP_DIR} --output appimage