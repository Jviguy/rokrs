# rokrs
A small program for controlling locally hosted Roku devices.

## Usage
To run rokrs: `./rokrs` or `cargo install --path . && rokrs` for a global install.

Once the program is opened you should be welcomed by a message stating that the local network is going to be scanned.

After scanning is complete you will see a numbered list of devices that exist on your network.

It will then prompt you to enter a number and it will connect you to that given device.

From there you will be in the rokrs control terminal.

### Commands
- keypress
  - Used to do a full keypress interaction e.g keydown and keyup
  - example: `keypress PowerOff`
  - for a list of valid keys view the [ECP API Page](https://developer.roku.com/docs/developer-program/debugging/external-control-api.md) it will be under keypress values.
- launch
  - Used to launch a given app by its id
  - example: `launch 8378` launches the mlb channel
- install
  - used to install a given app by its id
  - exmple: `install 8378`
- rescan
  - rescans the local network and allows you to pick a different device in the control terminal
  - example: `rescan`
- exit
  - exits the rokrs exe
  - example: `exit`
