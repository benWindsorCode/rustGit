pub struct Config {
    path: String,
    contents: ConfigContents
}

struct ConfigContents {

}

impl Config {
    pub fn new(path: String) -> Self {
        Config { path, contents: ConfigContents{} }
    }
}