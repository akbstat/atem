const express = require("express");
const app = express();
const fs = require("fs");
const cors = require("cors");
const filepath = "D:\\projects\\rusty\\atem\\render\\static\\annotation.js";

app.use(express.json());
app.use(cors());
app.use(express.static("static"));

app.get("/", (req, res) => {
    res.sendFile("index.html");
});

app.post("/api", (req, res) => {
    const bytes = req.body;
    const data = `const data = ${JSON.stringify(bytes)};`;
    fs.writeFileSync(filepath, data)
    res.send("done")
})

app.get("/api/llm", (req, res) => {
    const result = fs.readFileSync("D:\\projects\\rusty\\atem\\render\\static\\cheat.json");
    setTimeout(() => { console.log("invoke llm"); res.json(JSON.parse(result)) }, 5000);
})

app.listen(3000, () => {
    console.log("listening port 3000");
});
