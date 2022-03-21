# spuristo

spuristo is a CLI tracking application that utilizes bluetooth and linear regression to guess the number of people in the vicinity of a device.

This is a project which has a lot of work that remains to be done at the time of writing this.

## Get started

todo™️

## Options

```
--adapter		Specify the name of the bluetooth adapter that will be used.
			Default is set to "hci0". Do not touch if you don't know what you're doing.

--threshold		Specify the minimum connection strength in dBm required to "count" a device.
			Default is set to -75 dBm

--training		Run the application in training mode. This will enter all datapoints into a
			separate database table used for training the model when running in production.
```

### A note about CLI arguments

There's a known issue about the `--threshold` argument where inputing negative numbers are not properly supported. In order to mitigate this, you need to use the following syntax

```
spuristo --threshold=-75
```

This is detailed in issue #3

## Contributite

This project is meant to be a fun hobby project allowing me to solve the problem of not knowing how many people are at the place I hang out at school, while also giving me an excuse to learn Rust. Therefore I wish to keep contributions to a minimal in order to force myself to learn and figure out the ways of Rust. The exceptions to this are issues marked with the _Help wanted_ tag.
