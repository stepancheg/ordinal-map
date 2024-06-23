Generate README.md from doc comments

Usage: cargo-readme readme [OPTIONS]

Options:
      --no-badges            Do not prepend badges line. By default, badges defined in Cargo.toml are prepended to the output. Ignored when using a template
      --no-indent-headings   Do not add an extra level to headings. By default, '#' headings become '##', so the first '#' can be the crate name. Use this option to prevent this behavior
      --no-license           Do not append license line. By default, the license defined in `Cargo.toml` will be prepended to the output. Ignored when using a template
      --no-template          Ignore template file when generating README. Only useful to ignore default template `README.tpl`
      --no-title             Do not prepend title line. By default, the title ('# crate-name') is prepended to the output
  -i, --input <INPUT>        File to read from. If not provided, will try to use `src/lib.rs`, then `src/main.rs`. If neither file could be found, will look into `Cargo.toml` for a `[lib]`, then for a single `[[bin]]`. If multiple binaries are found, an error will be returned
  -o, --output <OUTPUT>      File to write to. If not provided, will output to stdout
  -r, --project-root <ROOT>  Directory to be set as project root (where `Cargo.toml` is) Defaults to the current directory
  -t, --template <TEMPLATE>  Template used to render the output. Default behavior is to use `README.tpl` if it exists
  -h, --help                 Print help
  -V, --version              Print version
