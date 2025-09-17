import init, { parse_from_memory } from "../../pkg/esm_parser.js";

async function run() {
  await init();

  const filePath = "/examples/data/TestTachoData.DDD";

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
    const result = await parse_from_memory(data);
    console.log("Success! Parsed data:", JSON.stringify(result, null, 2));
  } catch (error) {
    console.error("An error occurred:", error);
  }
}

run();
