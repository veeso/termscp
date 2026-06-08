# Notifications

termscp sends desktop notifications for the following events:

- **Transfer completed**: sent once a transfer has been successfully completed.
  Displayed only if the total transfer size is at least the configured
  `Notifications: minimum transfer size`.
- **Transfer failed**: sent once a transfer has failed due to an error.
  Displayed only if the total transfer size is at least the configured
  `Notifications: minimum transfer size`.
- **Update available**: sent whenever a new version of termscp is available.
- **Update installed**: sent whenever a new version of termscp has been
  installed.
- **Update failed**: sent whenever the installation of an update fails.

## Disable notifications

To turn notifications off, enter setup and set `Enable notifications?` to `No`.

## Change the minimum transfer size

To change the threshold that gates transfer notifications, enter setup and set
`Notifications: minimum transfer size` to the value that suits you.
