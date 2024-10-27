/// Install pkg-config on MacOSX (before you install redis)
/// Download from: https://pkg-config.freedesktop.org/releases/
/// Step 1: curl http://pkgconfig.freedesktop.org/releases/pkg-config-0.29.2.tar.gz -o pkg-config-0.29.2.tar.gz
/// Step 2: tar -xf pkg-config-0.29.2.tar.gz
/// Step 3: cd pkg-config-0.29.2
/// Step 4: ./configure  --with-internal-glib  (if you got an `incompatible integer to pointer conversion` issue)
/// Step 4: CFLAGS="-Wno-int-conversion" CXXFLAGS="-Wno-int-conversion" ./configure --prefix="$3" --with-pc-path="$3/lib/pkgconfig" --with-internal-glib
/// Step 5: make
/// Step 6: sudo  make install
/// If you got an `incompatible integer to pointer conversion` issue: https://gitlab.freedesktop.org/pkg-config/pkg-config/-/issues/81
fn install_pkg_config() {
    println!("install pkg-config");
}

/// Install redis from source code
/// Download from: https://redis.io/docs/latest/operate/oss_and_stack/install/install-redis/install-redis-from-source/
/// Step 1: wget https://download.redis.io/redis-stable.tar.gz
/// Step 2: tar -xzvf redis-stable.tar.gz
/// Step 3: cd redis-stable
/// Step 4: make
/// Step 5: make BUILD_TLS=yes  # Optional if TLS is need
/// Step 6: sudo make install
fn install_redis_server() {
    println!("install redis-server");
}


fn main() {}