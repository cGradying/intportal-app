# Student data stays on the device until the cloud tier

Schedule, grades, marks, subject colours, notes and decks live in one SQLite file on the device. SIS credentials live in the iOS Keychain or the Android Keystore. There is no backend and no sync between devices.

A backend arrives only with the planned paid cloud tier, which brings cloud IntAssis and sync. Until then, nothing is built "for sync later". The reasons are PUP's own-account-only terms, the privacy obligations that come with students' grades on a server, and cost.
