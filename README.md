# macos-emond
A simple macOS Emond parser (and very simple library) written in Rust!  

# Emond
Emond (Event Monitor Daemon) is an event service that lets a user create rules/scripts to execute tasks. Currently there are four (4) tasks that can be executved via Emond:  
+ Log - Log an event
+ Send Email - Send an email
+ Send SMS - Send a SMS message
+ Run Command - Execute a command

All Emond rules/scripts are XML PLIST files.

Emond can be abused for malicious activity by being used to Persist on a macOS system.  This simple Rust program parsers several components related to Emond:
+ Parse the Emond config PLIST file at `/etc/emond.d/emond.plist`. This PLIST file contains a list of directories that point to where Emond looks for Emond rules/scripts.
+ Parse all PLIST files found Emond rules/scripts directories defined in `/etc/emond.d/emond.plist`. By default Emond checks the directory `/etc/emond.d/rules`
  + A default sample rules named `SampleRules.plist` is found on most modern macOS systems. This rule/script is disabled.
+ In order for Emond daemon to be active a file must exist at `/private/var/db/emondClients`. If the directory `/private/var/db/emondClients` is empty, Emond will not start. This program checks for the presence of any files in the directory `/private/var/db/emondClients`

# References
https://www.xorrior.com/emond-persistence/  
https://magnusviri.com/what-is-emond.html