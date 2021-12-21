VSCode guide:
  * install the [extension](https://marketplace.visualstudio.com/items?itemName=emeraldwalk.RunOnSave) 
  * setup `PATH` to `toml_fmt`
  * configure the settings:
```json
"emeraldwalk.runonsave": {
  "commands": [
      {
          "match": "\\.toml$",
          "cmd": "toml_fmt ${file}"
      }
  ]
}
```
    
