#!/usr/bin/env node

const { exec } = require("child_process");

exec(`alias gg="ggfb"`, (error, stdout, stderr) => {
    console.log(stdout);
    if (error || stderr) {
        console.log(error || stderr);
    } else {
        console.log("alias set to gg. try typing 'gg help' for available commands");
    }
})

