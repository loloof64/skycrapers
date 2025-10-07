# Use a stable, modern Ubuntu LTS image.
FROM ubuntu:22.04

# Avoid prompts during package installation.
ENV DEBIAN_FRONTEND=noninteractive

# Define environment variables early.
ENV APP=skyscrapers
ENV APP_DIR=${APP}.AppDir

# Install essential build tools and dependencies.
RUN apt update && apt install -y \
    wget \
    build-essential \
    curl \
    git \
    libgtk-3-dev \
    libgdk-pixbuf-2.0-dev \
    pkg-config \
    libgtk-3-bin \
    file \
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
RUN cargo build --release

#--------------------------------------------------------------------------
# AppImage Creation Phase (Optimized for GTK Stability)
#--------------------------------------------------------------------------

# 1. Download only the core linuxdeploy tool.
RUN wget -c -O linuxdeploy https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage \
    && chmod +x linuxdeploy

# 2. Extract linuxdeploy to bypass FUSE requirements during the build.
RUN ./linuxdeploy --appimage-extract

# Prepare AppDir structure.
RUN mkdir -p ${APP_DIR}/usr/bin

# Copy the compiled binary.
RUN cp target/release/skyscrapers ${APP_DIR}/usr/bin/skyscrapers
# Explicitly set execute permissions.
RUN chmod +x ${APP_DIR}/usr/bin/skyscrapers

# --- GTK Cache Generation (Using Verified Path) ---
# a) Copy the full GTK modules and files directory 
RUN mkdir -p ${APP_DIR}/usr/lib/gtk-3.0 \
    && cp -r /usr/lib/x86_64-linux-gnu/gtk-3.0/* ${APP_DIR}/usr/lib/gtk-3.0/
# b) Generate the GDK-Pixbuf loader cache. 
RUN mkdir -p ${APP_DIR}/usr/lib/gdk-pixbuf-2.0/2.10.0 \
    && /usr/lib/x86_64-linux-gnu/gdk-pixbuf-2.0/gdk-pixbuf-query-loaders > ${APP_DIR}/usr/lib/gdk-pixbuf-2.0/2.10.0/loaders.cache
# c) Generate the GTK Input Method cache. 
# Using the verified path /usr/lib/x86_64-linux-gnu/libgtk-3-0/gtk-query-immodules-3.0
RUN mkdir -p ${APP_DIR}/usr/lib/gtk-3.0/3.0.0 \
    && /usr/lib/x86_64-linux-gnu/libgtk-3-0/gtk-query-immodules-3.0 > ${APP_DIR}/usr/lib/gtk-3.0/3.0.0/immodules.cache

# 2. Create the robust AppRun script.
RUN echo '#!/bin/sh' > ${APP_DIR}/AppRun \
    && echo 'export GDK_PIXBUF_MODULE_FILE="$APPDIR/usr/lib/gdk-pixbuf-2.10.0/loaders.cache"' >> ${APP_DIR}/AppRun \
    && echo 'export GTK_IM_MODULE_FILE="$APPDIR/usr/lib/gtk-3.0/3.0.0/immodules.cache"' >> ${APP_DIR}/AppRun \
    && echo 'exec "$APPDIR"/usr/bin/skyscrapers "$@"' >> ${APP_DIR}/AppRun \
    && chmod +x ${APP_DIR}/AppRun

# Place the .desktop file in the mandatory usr/share/applications path.
RUN mkdir -p ${APP_DIR}/usr/share/applications/
# Using precise 'echo' commands to prevent any trailing whitespace on the Icon line.
RUN echo "[Desktop Entry]" > ${APP_DIR}/usr/share/applications/${APP}.desktop \
    && echo "Version=1.0" >> ${APP_DIR}/usr/share/applications/${APP}.desktop \
    && echo "Type=Application" >> ${APP_DIR}/usr/share/applications/${APP}.desktop \
    && echo "Name=Skyscrapers" >> ${APP_DIR}/usr/share/applications/${APP}.desktop \
    && echo "Exec=skyscrapers" >> ${APP_DIR}/usr/share/applications/${APP}.desktop \
    && echo "Icon=skyscrapers" >> ${APP_DIR}/usr/share/applications/${APP}.desktop \
    && echo "Categories=Game;" >> ${APP_DIR}/usr/share/applications/${APP}.desktop

# FIX 2: Icon copying (Two essential copies remaining).
# Copy 1: To the root of the AppDir (The file appimagetool explicitly demands when searching defaults)
COPY skyscrapers.png ${APP_DIR}/skyscrapers.png
# Copy 2: To the standard icon path within the AppDir (The file linuxdeploy uses)
RUN mkdir -p ${APP_DIR}/usr/share/icons/hicolor/256x256/apps/
COPY skyscrapers.png ${APP_DIR}/usr/share/icons/hicolor/256x256/apps/skyscrapers.png


# Initialize the library path variable (Fixes the UndefinedVar warning)
ENV LD_LIBRARY_PATH=

# Set the library path to assist linuxdeploy in finding all necessary ".so" files.
ENV LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/usr/lib/x86_64-linux-gnu

# 3. Final step: Execute linuxdeploy. 
RUN squashfs-root/AppRun --appdir ${APP_DIR} --output appimage