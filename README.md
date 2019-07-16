# MCMGMT (JAMP MC)
> Just Another (Minecraft) Management Panel

# Supported only on MacOS and Linux

# About
MCMGMT Aims to be easy to use. The goals of MCMGMT are as follows:
* Intuitive user interface
* Easy to setup (portable)
* Simple to remove
* Avaliable as standalone or connected to the network
* Easily dockerize your servers
* Accessable

## What it's built with
MCMGMT is built with rust for its high speed and memory security with little to no overhead.
MCMGMT uses bincode to serialize messages for its speed and low size.

## How it is structured
### core
The definitions used both on the client and server to communicate with eachother
### Backend
The backend server running at mc.mgmt.dusterthefirst.com aswell as the server that will run on the host if you are running in standalone mode
### CLI
The command line tool to setup and manage MCMGMT
### Manage-d
The management daemon running on the host. Its job is to manage the docker instances of the different minecraft servers aswell as communicate to the server be it local or remote
### Web
The website that will be served from the server if installed or visible at mc.mgmt.dusterthefirst.com


## Protocol
* cli <--> manage-d use raw tcp on port 9895
* manage-d <--> backend use raw tcp on port **TBD**
* backend <--> web use websockets on port **TBD**
> Definitions for all packets can be found in the *core* crate


# Roadmap
1. It works
2. Install instructions for linux and macos
    1. Support for major distros
        1. Ubuntu
        2. Debian
    2. Support for major init systems
        1. SystemD
        2. System V
        3. Upstart
        4. OpenRC
        5. Runit
        6. ?
3. Install script
4. Upgrade script
5. Package into .deb/other package
5. In place update/unattended upgrades

# License
GNU GPL Version 3.0

