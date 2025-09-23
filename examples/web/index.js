import init, { parse_from_memory } from "../../pkg/esm_parser.js";
import { DisplayData } from "./display.js";

async function run() {
  await init();

  const filePath = "/examples/data/Card0004.DDD";

  try {
    const rootEl = document.getElementById("root-container");
    console.log(`Fetching data from ${filePath}...`);
    const response = await fetch(filePath);

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    const arrayBuffer = await response.arrayBuffer();
    const data = new Uint8Array(arrayBuffer);

    console.log(`Successfully loaded ${data.byteLength} bytes. Calling parse_from_memory...`);
    const start = performance.now();
    const result = await parse_from_memory(data);
    const display = new DisplayData();
    display.applyData(rootEl, result);
    const end = performance.now();
    console.log(`Execution time: ${end - start} ms`);
  } catch (error) {
    console.error("An error occurred:", error);
  }
}

run();
