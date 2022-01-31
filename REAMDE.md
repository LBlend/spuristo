# spuristo

spuristo is a CLI tracking application that utilizes bluetooth and logistic regression to guess the number of people in the vicinity of a device. It sports an API for fetching data and aims to be configurable to suit your needs.

## Get started

todo™️

## Options

```
--threshold		Specify the minimum connection strength in dBm required to "count" a device.
				Default is set to -75 dBm
--training		Run the application in training mode. This will enter all datapoints into a
				separate database table used for training the model when running in production.
--adapter		Specify the name of the bluetooth adapter that will be used.
				Default is set to "hci10". Do not touch if you don't know what you're doing.
```


