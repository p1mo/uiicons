<center><h1>UIIcons</h1></center>
<center><h3>version: <code>0.1.0</code></h3></center>
<center><h5>serve icons from various sources to the frontend</h5></center>

### Features
+ [X] Embeded
+ [X] Vanilla Component (Custom Component API)
+ [X] Parallel Icon finder for Large Icon Sets

### Example

**build.rs**
```rust
fn main() {
    uiicons::build().unwrap();
}
```

**uiicons.json**
```json
{
  "$schema": "gen/uiicons.schema.json",
  "component": {
    "name": "ui-icon",
    "icon": "home",
    "size": "24px",
    "kind": "stroke"
  },
  "icons": {
    "shield": {
      "source": "phosphor",
      "icon": "shield",
      "kind": "light"
    },
    "action_123": {
      "source": "google",
      "icon": "action/123",
      "kind": "sharp"
    }
  }
}
```

**main.rs**
```rust
fn main() {
    // Optional: with feature flag `js` serve component to frontend
    let js_bytes = uiicons::embeded_js();
    
    // icon holder
    let icons = uiicons::embeded_icons();

    // get icon set length
    let length = icons.len();

    // get icon and serve the vec<u8>
    let svg = icons.get("my_icon");

    // find icon and serve the vec<u8>
    let svg = icons.find("my_icon");

    // find icon in parallel and serve the vec<u8>
    let svg = icons.par_find("my_icon");

    // iter over icon set and serve the vec<u8>
    // for par iter add rayon crate and use rayon's par_bridge
    for data in icons.iter() {}
}
```

### List of Sources
 
| IconSet           | Website                           | Github                                            |
|:------------------|:----------------------------------|:--------------------------------------------------|
| tabler            | https://tabler.io                 | https://github.com/tabler/tabler-icons            |
| simpleicons       | https://simpleicons.org           | https://github.com/simple-icons/simple-icons      |
| feather           | https://feathericons.com/         | https://github.com/feathericons/feather           |
| google            | https://fonts.google.com/icons    | https://github.com/google/material-design-icons   |
| lucide            | https://lucide.dev                | https://github.com/lucide-icons/lucide            |
| heroicons         | https://heroicons.com             | https://github.com/tailwindlabs/heroicons         |
| bootstrap         | https://icons.getbootstrap.com    | https://github.com/twbs/icons                     |
| remixicon         | https://remixicon.com             | https://github.com/Remix-Design/RemixIcon         |
| iconoir           | https://iconoir.com               | https://github.com/iconoir-icons/iconoir          |
| phosphor          | https://phosphoricons.com         | https://github.com/phosphor-icons/core            |
| thesvg            | https://thesvg.org                | https://github.com/glincker/thesvg                |
| devicons          | https://devicon.dev               | https://github.com/devicons/devicon               |