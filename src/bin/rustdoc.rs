use cargo::ops;
use cargo::util::{CliResult, Config};
use cargo::util::important_paths::{find_root_manifest_for_wd};

#[derive(RustcDecodable)]
pub struct Options {
    arg_opts: Vec<String>,
    flag_target: Option<String>,
    flag_features: Vec<String>,
    flag_jobs: Option<u32>,
    flag_manifest_path: Option<String>,
    flag_no_default_features: bool,
    flag_open: bool,
    flag_verbose: bool,
    flag_release: bool,
    flag_quiet: bool,
    flag_color: Option<String>,
    flag_package: Option<String>,
    flag_lib: bool,
    flag_bin: Vec<String>,
    flag_example: Vec<String>,
    flag_test: Vec<String>,
    flag_bench: Vec<String>,
}

pub const USAGE: &'static str = "
Build a package's documentation, using specified custom flags.

Usage:
    cargo rustdoc [options] [--] [<opts>...]

Options:
    -h, --help               Print this message
    --open                   Opens the docs in a browser after the operation
    -p SPEC, --package SPEC  Package to document
    -j N, --jobs N           The number of jobs to run in parallel
    --lib                    Build only this package's library
    --bin NAME               Build only the specified binary
    --example NAME           Build only the specified example
    --test NAME              Build only the specified test target
    --bench NAME             Build only the specified benchmark target
    --release                Build artifacts in release mode, with optimizations
    --features FEATURES      Space-separated list of features to also build
    --no-default-features    Do not build the `default` feature
    --target TRIPLE          Build for the target triple
    --manifest-path PATH     Path to the manifest to document
    -v, --verbose            Use verbose output
    -q, --quiet              No output printed to stdout
    --color WHEN             Coloring: auto, always, never

The specified target for the current package (or package specified by SPEC if
provided) will be documented with the specified <opts>... being passed to the
final rustdoc invocation. Dependencies will not be documented as part of this
command.  Note that rustdoc will still unconditionally receive arguments such
as -L, --extern, and --crate-type, and the specified <opts>...  will simply be
added to the rustdoc invocation.

If the --package argument is given, then SPEC is a package id specification
which indicates which package should be documented. If it is not given, then the
current package is documented. For more information on SPEC and its format, see
the `cargo help pkgid` command.
";

pub fn execute(options: Options, config: &Config) -> CliResult<Option<()>> {
    try!(config.shell().set_verbosity(options.flag_verbose, options.flag_quiet));
    try!(config.shell().set_color_config(options.flag_color.as_ref().map(|s| &s[..])));

    let root = try!(find_root_manifest_for_wd(options.flag_manifest_path,
                                              config.cwd()));

    let doc_opts = ops::DocOptions {
        open_result: options.flag_open,
        compile_opts: ops::CompileOptions {
            config: config,
            jobs: options.flag_jobs,
            target: options.flag_target.as_ref().map(|t| &t[..]),
            features: &options.flag_features,
            no_default_features: options.flag_no_default_features,
            spec: &options.flag_package.map_or(Vec::new(), |s| vec![s]),
            exec_engine: None,
            release: options.flag_release,
            filter: ops::CompileFilter::new(options.flag_lib,
                                            &options.flag_bin,
                                            &options.flag_test,
                                            &options.flag_example,
                                            &options.flag_bench),
            mode: ops::CompileMode::Doc { deps: false },
            target_rustdoc_args: Some(&options.arg_opts),
            target_rustc_args: None,
        },
    };

    try!(ops::doc(&root, &doc_opts));

    Ok(None)
}
