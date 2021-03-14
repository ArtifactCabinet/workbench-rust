# Laugh in rust
![rust_meme](https://i.redd.it/7bbj3cznnks51.png)



# Tooling

vs code-runner

```json
    "code-runner.executorMap": {
        "rust": "cd $dir && rustc $fileName -o $fileNameWithoutExt.exe && $dir$fileNameWithoutExt",
    }
```