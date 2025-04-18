# POWER
this repository provides both 
1. modular library for interfacing with powered devices
2. CLI application which uses this library to control powered devices from the command line

**IMPORTANT:** this is a work in progress, and the API will likely have breaking changes as it grows. ADDITIONALLY, currently there is only trait implementations for Tasmota outlets, however other devices can be supported by implementing these traits for your device-specific API. You can use `tasmota.rs` as an example of how to do this. once implemented, your devices will be compatible with this library. 

# CLI
## BUILDING THE CLI
with a working rust toolchain on your machine, run the following: 

`cargo build --release --bin cli`

once built, the CLI binary will be available at the following path: 
`<project root>/target/release/cli`

## CONFIGURING THE SYSTEM DESCRIPTION FILE
a `system` consists of a number of `components` which are defined in a configuration `toml` file. 

an example, `config.toml` has been provided. 

each entry corresponds to a `component` and provides a `name` as well as the `target` IP address of the (in this case tasmota) power device. 

```toml
components = [
    {name="OUTLET 1", target="OUTLET_1_IP"}, # OUTLET 1
    {name="OUTLET 2", target="OUTLET_2_IP"}, # OUTLET 2
    {name="OUTLET 3", target="OUTLET_3_IP"}, # OUTLET 3 ...
]
```

## RUNNING THE CLI
the CLI takes as an argument the `config-file` (described above) as well as a subcommand. 

more usage information about the CLI can be obtained by running

`./cli help`

## COMMANDS
### MONITOR
the `monitor` command will output the power consumption of the components listed in the config file. 

example: 
```log
OUTLET_1
* POWER: 31W
------------------
OUTLET_2
* POWER: 0W
------------------
OUTLET_3
* POWER: 0W
------------------
```

### SET
the `set` command allows a user to set the power state of a `component` at a given `index`. `component`s are indexed starting at 0 in the order they are defined in your configuration file. 