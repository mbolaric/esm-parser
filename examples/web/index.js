import init, { parse_from_memory } from "../../pkg/esm_parser.js";
import { DisplayData } from "./display.js";

async function run() {
  await init();

  const filePath = "/examples/data/Card0004.DDD";
  const display = new DisplayData();

  try {
    const rootEl = document.getElementById("root-container");
    const inputFile = document.getElementById("file-upload");
    const fileName = document.getElementById("file-name");
    const stattusBar = document.getElementById("shell.bottom.footer.left.status.link");


    inputFile.addEventListener("change", (e) => {
      const file = e.target.files[0];
      if (!file) return;

      const reader = new FileReader();
      reader.onload = async (event) => {
        const arrayBuffer = event.target.result;
        const data = new Uint8Array(arrayBuffer);
        const result = await parse_from_memory(data);
        display.applyData(rootEl, result, file.name);
        fileName.innerText = file.name;
        stattusBar.innerHTML = `Full file name: ${file.name}`;
      };

      reader.readAsArrayBuffer(file);
    });
  } catch (error) {
    console.error("An error occurred:", error);
  }
}

run();
