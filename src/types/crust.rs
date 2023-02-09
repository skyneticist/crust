pub struct CrustConfig<'a> {
    pub verbosity: Option<u8>,
    pub hide: Option<bool>,
    pub dump_location: Option<&'a str>,
    pub ee_img_path: Option<&'a str>,
}

impl CrustConfig<'_> {
    pub fn set() -> Self {
        CrustConfig {
            verbosity: verbosity,
            hide: hide,
            dump_location: dump_location,
            ee_img_path: ee_img_path,
        }
    }
}

pub struct Crust<'a> {
    pub config: CrustConfig<'a>,
}

impl Crust<'_> {
    pub fn new() -> Self {
        let config = CrustConfig::set();
        Crust { config }
    }
    pub fn run() {
        Crust::new();

        let args: Cli = Cli::from_args();
        let sub_cmd = args.arg.unwrap_or_else(|| "".to_string());
        let output = match args.command {
            x if x == Acp.value() || x == Acp.short_value() => Acp.method(sub_cmd),
            x if x == Cloud.value() || x == Cloud.short_value() => Cloud.method(sub_cmd),
            x if x == Cob.value() || x == Cob.short_value() => Cob.method(sub_cmd),
            x if x == DevOps.value() || x == DevOps.short_value() => DevOps.method(sub_cmd),
            x if x == PullAll.value() || x == PullAll.short_value() => PullAll.method(sub_cmd),
            x if x == Help.value() || x == Help.short_value() => Help.method(sub_cmd),
            x if x == Log.value() || x == Log.short_value() => Log.method(sub_cmd),
            x if x == Go.value() || x == Go.short_value() => Go.method(sub_cmd),
            x if x == So.value() || x == So.short_value() => So.method(sub_cmd),
            x if x == SoftReset.value() || x == SoftReset.short_value() => {
                SoftReset.method(sub_cmd)
            }
            x if x == Status.value() || x == Status.short_value() => Status.method(sub_cmd),
            x if x == "*" => display_msg(),
            _ => Help.method(sub_cmd),
        };
        println!("{}", output);
    }
}

#[derive(StructOpt)]
pub struct Cli {
    command: String,
    arg: Option<String>,
}

fn display_msg() -> String {
    String::from(" Welcome to Crust \u{1F35E} \nTry typing `crust help`")
        .green()
        .to_string()
}
