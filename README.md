# PlumeImpactor

[![GitHub Release](https://img.shields.io/github/v/release/khcrysalis/PlumeImpactor?include_prereleases)](https://github.com/khcrysalis/PlumeImpactor/releases)
[![GitHub License](https://img.shields.io/github/license/khcrysalis/PlumeImpactor?color=%23C96FAD)](https://github.com/khcrysalis/PlumeImpactor/blob/main/LICENSE)
[![Sponsor Me](https://img.shields.io/static/v1?label=Sponsor&message=%E2%9D%A4&logo=GitHub&color=%23fe8e86)](https://github.com/sponsors/khcrysalis)

PlumeImpactor is an open-source alternative to tools like CydiaImpactor, Sideloadly, and AltStore. Made for sideloading to iOS / tvOS devices.

## Structure

The project is seperated in multiple modules, all serve single or multiple uses depending on their importance.

| Module               | Description                                                                                                                   |
| -------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `apps/plumeimpactor` | GUI interface for the crates shown below, backend using wxWidgets (with a rust ffi wrapper, wxDragon)                         |
| `apps/plumesign`     | CLI interface for the crates shown below, using `clap`.                                                                       |
| `crates/grand_slam`  | Handles all api request used for communicating with Apple developer services, along with providing auth for Apple's grandslam |
| `crates/ldid2`       | Wrapper for applecodesign-rs with additional features, specifically made to support iOS sideloading and app modifications     |

## Acknowledgements

- [Samara](https://github.com/khcrysalis) - ME!
- [apple-private-apis](https://github.com/SideStore/apple-private-apis) - Grandslam auth & Omnisette.
- [apple-codesign-rs](https://github.com/indygreg/apple-platform-rs) - Open-source alternative to codesign.
- [idevice](https://github.com/jkcoxson/idevice) - Used for communication with `installd`, specifically for sideloading the apps to your devices.
