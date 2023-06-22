const fs = require("fs");

// package the manifest
const { version } = JSON.parse(fs.readFileSync("./package.json"));
const package = JSON.parse(fs.readFileSync("./resources/package.json"));
fs.writeFileSync(
    "./pkg/package.json",
    JSON.stringify(
        {
            ...package,
            version,
        },
        null,
        2
    )
);

// package the types
fs.writeFileSync("./pkg/types.d.ts", fs.readFileSync("./resources/types.d.ts"));
