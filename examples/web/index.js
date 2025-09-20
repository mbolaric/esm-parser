import init, { parse_from_memory } from "../../pkg/esm_parser.js";

async function run() {
  await init();

  const filePath = "/examples/data/Card0004.DDD";

  try {
    console.log(`Fetching data from ${filePath}...`);
    const response = await fetch(filePath);

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    const arrayBuffer = await response.arrayBuffer();
    const data = new Uint8Array(arrayBuffer);

    console.log(
      `Successfully loaded ${data.byteLength} bytes. Calling parse_from_memory...`,
    );
    const start = performance.now();
    const result = await parse_from_memory(data);
    const end = performance.now();
    console.log(`Execution time: ${end - start} ms`);
    console.log("Success! Parsed data:", JSON.stringify(result, null, 2));
  } catch (error) {
    console.error("An error occurred:", error);
  }
}

run();
