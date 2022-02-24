## SQL2CSV

A simple command line tool to convert SQL database output to CSV.

### Help

```bash
sql2csv 0.1.0
A simple command line tool to convert SQL database output to CSV

USAGE:
    sql2csv [OPTIONS]

OPTIONS:
    -e, --exclude-header    Whether or not the header row should be included, defaults to true
    -h, --help              Print help information
    -V, --version           Print version information
```

Suggested usage is to either pipe the tabular database output into the executable, or run the executable directly and paste in the tabular data. Output is printed to stdout.