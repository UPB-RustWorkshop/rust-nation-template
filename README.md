# Tock Tutorial Rust Nation UK

The purpose of this turtorial is to build a simple smarthome device.

To work, the smarthome device has the folllowing components:
1. a smarthome kernel driver that will act as the data model
2. a smarthome service that will pull the data from the sensors (doors,
  temperature) and send the it to the driver for storing
3. a ui app that displays the doors status and the temperature

For this tutorial you will have to implements the driver (1) and
the service (2). The UI app (3) is already implemented using
the C/C++ language.

## Driver
Write a Tock driver for the smarthome.

### Write the driver

### Register the driver

## Service (Rust)
Write a rust application that implements the logic of the smart home.

### Driver API

### Service

#### Read the buttons

#### Reac the temperature


