# `PineTime App In Rust`

> This project was intended to be a starter project to learn the Rust programming language
 
 ## VS Code Requirements
 * 


## To enable Interrupts:
From the [Rust Embedded Book](https://rust-embedded.github.io/book/start/interrupts.html):

```the cortex-m-rt crate provides an interrupt attribute to declare interrupt handlers```

From the [cortex-m-rt docs](https://docs.rs/cortex-m-rt/0.6.13/cortex_m_rt/attr.interrupt.html):

```[the interrupt] attribute is exposed by cortex-m-rt only when the device feature is enabled```

To enable the interrupt attribute, be sure the following is in the cargo.Toml:
```
[dependencies.nrf52832-pac]
version = "0.9.0"
features = ["rt"]

[dependencies.cortex-m-rt]
version = "0.6.13"
features = ["device"]
```

The interrupt attribute is included by :

``` use nrf52832-pac::interrupt```
