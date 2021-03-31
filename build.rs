fn main() {
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++11")
        .include("gmod-module-base/include/GarrysMod/Lua")
        .file("src/bridge.cpp")
        .compile("bridge");
}
