use static_files::NpmBuild;


fn main() {
    println!("cargo:rerun-if-changed=./web");
    NpmBuild::new("./web")
        .install()
        .unwrap()
        .run("build")
        .unwrap()
        .target("./web/build")
        .to_resource_dir()
        .build()
        .unwrap();
}
